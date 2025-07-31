use crate::bus::Bus;
use rodio::{OutputStream, Sink, Source};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration; // Import Bus
const PERIOD_TABLE: [u16; 16] = [
    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
];
struct SoundChannel {
    frequency: Arc<Mutex<f32>>,
    volume: Arc<Mutex<f32>>,
    enabled: Arc<Mutex<bool>>,
    length_counter: Arc<Mutex<u8>>,
    length_counter_enabled: Arc<Mutex<bool>>,
    length_counter_halt: Arc<Mutex<bool>>,
    sweep_mute: Arc<Mutex<bool>>,
}

impl SoundChannel {
    fn new() -> Self {
        SoundChannel {
            frequency: Arc::new(Mutex::new(0.0)),
            volume: Arc::new(Mutex::new(0.0)),
            enabled: Arc::new(Mutex::new(false)),
            length_counter: Arc::new(Mutex::new(0)),
            length_counter_enabled: Arc::new(Mutex::new(true)),
            length_counter_halt: Arc::new(Mutex::new(false)),
            sweep_mute: Arc::new(Mutex::new(false)),
        }
    }

    fn set_frequency(&self, freq: f32) {
        *self.frequency.lock().unwrap() = freq;
    }

    fn set_volume(&self, vol: f32) {
        *self.volume.lock().unwrap() = vol;
    }

    fn set_enabled(&self, enabled: bool) {
        *self.enabled.lock().unwrap() = enabled;

        if !enabled {
            *self.length_counter.lock().unwrap() = 0;
        }
    }

    fn set_length_counter(&self, value: u8) {
        *self.length_counter.lock().unwrap() = value;
    }

    fn set_length_counter_halt(&self, halt: bool) {
        *self.length_counter_halt.lock().unwrap() = halt;
    }

    fn decrement_length_counter(&self) -> bool {
        let mut counter = self.length_counter.lock().unwrap();
        let halt = *self.length_counter_halt.lock().unwrap();
        let enabled = *self.length_counter_enabled.lock().unwrap();

        if halt || !enabled {
            return *counter > 0;
        }

        if *counter > 0 {
            *counter -= 1;
            return *counter > 0;
        }

        false
    }
}

struct SweepUnit {
    divider_period: u8,
    negate_flag: bool,
    shift_amount: u8,
    enabled: bool,
    divider_counter: u8,
    mute: Arc<Mutex<bool>>,
}

impl SweepUnit {
    fn new(channel_sweep_mute: Arc<Mutex<bool>>) -> Self {
        SweepUnit {
            divider_period: 0,
            negate_flag: false,
            shift_amount: 0,
            enabled: false,
            divider_counter: 0,
            mute: channel_sweep_mute,
        }
    }

    fn write(&mut self, data: u8) {
        self.enabled = (data & 0x80) != 0;
        self.divider_period = ((data >> 4) & 0x07) + 1;
        self.negate_flag = (data & 0x08) != 0;
        self.shift_amount = data & 0x07;
        self.reset_divider();
    }

    fn reset_divider(&mut self) {
        self.divider_counter = self.divider_period;
        *self.mute.lock().unwrap() = false;
    }

    fn clock(&mut self, current_timer: u16, channel_id: u8) -> Option<u16> {
        let mut _sweep_mute_local = false;
        let mut new_timer_value: Option<u16> = None;

        if self.divider_counter == 0 {
            self.divider_counter = self.divider_period;
            if self.enabled && self.shift_amount > 0 {
                let change = current_timer >> self.shift_amount;
                let calculated_new_timer = if self.negate_flag {
                    if channel_id == 1 {
                        current_timer.wrapping_sub(change).wrapping_sub(1)
                    } else {
                        current_timer.wrapping_sub(change)
                    }
                } else {
                    current_timer.wrapping_add(change)
                };

                if calculated_new_timer > 0x7FF || current_timer < 8 {
                    _sweep_mute_local = true;
                } else {
                    new_timer_value = Some(calculated_new_timer);
                    _sweep_mute_local = false;
                }
            } else {
                _sweep_mute_local = false;
            }
        } else {
            self.divider_counter -= 1;

            _sweep_mute_local = *self.mute.lock().unwrap();
        }
        *self.mute.lock().unwrap() = _sweep_mute_local;
        new_timer_value
    }
}

struct PulseWaveSource {
    channel: Arc<SoundChannel>,
    duty: Arc<Mutex<u8>>,
    sample_rate: u32,
    position: f32,
}

impl PulseWaveSource {
    fn new(channel: Arc<SoundChannel>, duty: Arc<Mutex<u8>>) -> Self {
        PulseWaveSource {
            channel,
            duty,
            sample_rate: 44100,
            position: 0.0,
        }
    }

    fn get_duty_output(&self, position: f32) -> f32 {
        let duty_cycle_val = *self.duty.lock().unwrap() & 0xC0;
        let duty_percent = match duty_cycle_val {
            0x00 => 0.125,
            0x40 => 0.25,
            0x80 => 0.5,
            0xC0 => 0.75,
            _ => 0.5,
        };

        if position < duty_percent {
            1.0
        } else {
            -1.0
        }
    }
}

impl Source for PulseWaveSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        44100
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for PulseWaveSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let freq = *self.channel.frequency.lock().unwrap();
        let vol = *self.channel.volume.lock().unwrap();
        let enabled = *self.channel.enabled.lock().unwrap();
        let length_counter = *self.channel.length_counter.lock().unwrap();
        let sweep_muted = *self.channel.sweep_mute.lock().unwrap();

        if !enabled || freq <= 0.0 || length_counter == 0 || sweep_muted {
            self.position = 0.0;
            return Some(0.0);
        }

        let sample = self.get_duty_output(self.position) * vol;

        self.position += freq / self.sample_rate as f32;

        while self.position >= 1.0 {
            self.position -= 1.0;
        }

        Some(sample)
    }
}

struct TriangleWaveSource {
    channel: Arc<SoundChannel>,
    sample_rate: u32,
    position: f32,
}

impl TriangleWaveSource {
    fn new(channel: Arc<SoundChannel>) -> Self {
        TriangleWaveSource {
            channel,
            sample_rate: 44100,
            position: 0.0,
        }
    }
}

impl Source for TriangleWaveSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        44100
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for TriangleWaveSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let freq = *self.channel.frequency.lock().unwrap();
        let vol = *self.channel.volume.lock().unwrap();
        let enabled = *self.channel.enabled.lock().unwrap();
        let length_counter = *self.channel.length_counter.lock().unwrap();

        if !enabled || freq <= 0.0 || length_counter == 0 {
            self.position = 0.0;
            return Some(0.0);
        }

        let sample = if self.position < 0.5 {
            (self.position * 4.0 - 1.0) * vol
        } else {
            (3.0 - self.position * 4.0) * vol
        };

        self.position += freq / self.sample_rate as f32;

        while self.position >= 1.0 {
            self.position -= 1.0;
        }

        Some(sample)
    }
}

struct NoiseSource {
    channel: Arc<SoundChannel>,
    mode_flag: Arc<Mutex<bool>>,
    sample_rate: u32,
    shift_register: u16,
    sample_period: f32,
    sample_timer: f32,
    current_output: f32,
}

impl NoiseSource {
    fn new(channel: Arc<SoundChannel>, mode_flag: Arc<Mutex<bool>>) -> Self {
        NoiseSource {
            channel,
            mode_flag,
            sample_rate: 44100,
            shift_register: 1,
            sample_period: 0.0,
            sample_timer: 0.0,
            current_output: 0.0,
        }
    }

    fn update_shift_register(&mut self) {
        let bit0 = self.shift_register & 1;

        let feedback_bit = if *self.mode_flag.lock().unwrap() {
            ((self.shift_register >> 6) & 1) ^ bit0
        } else {
            ((self.shift_register >> 1) & 1) ^ bit0
        };

        self.shift_register >>= 1;

        self.shift_register |= feedback_bit << 14;

        self.current_output = if bit0 == 0 { 1.0 } else { -1.0 };
    }
}

impl Source for NoiseSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        44100
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for NoiseSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let freq = *self.channel.frequency.lock().unwrap();
        let vol = *self.channel.volume.lock().unwrap();
        let enabled = *self.channel.enabled.lock().unwrap();
        let length_counter = *self.channel.length_counter.lock().unwrap();

        if !enabled || freq <= 0.0 || length_counter == 0 {
            return Some(0.0);
        }

        self.sample_timer += 1.0 / self.sample_rate as f32;

        self.sample_period = if freq > 0.0 { 1.0 / freq } else { 0.0 };

        if self.sample_period > 0.0 && self.sample_timer >= self.sample_period {
            self.sample_timer = 0.0;
            self.update_shift_register();
        }

        Some(self.current_output * vol)
    }
}

// DMC Source for rodio - will implement the actual fetch logic in APU
pub struct DmcSource {
    channel: Arc<SoundChannel>,
    dmc_output: Arc<Mutex<u8>>, // The current 7-bit DPCM output
    sample_rate: u32,
    timer_period: Arc<Mutex<u16>>, // Store the timer period for frequency calculation
    current_timer: u32,
    last_sample: f32,
}

impl DmcSource {
    fn new(
        channel: Arc<SoundChannel>,
        dmc_output: Arc<Mutex<u8>>,
        timer_period: Arc<Mutex<u16>>,
    ) -> Self {
        DmcSource {
            channel,
            dmc_output,
            sample_rate: 44100,
            timer_period,
            current_timer: 0,
            last_sample: 0.0,
        }
    }
}

impl Source for DmcSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for DmcSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let enabled = *self.channel.enabled.lock().unwrap();
        let dmc_output = *self.dmc_output.lock().unwrap();
        let timer_period = *self.timer_period.lock().unwrap();

        if !enabled {
            return Some(0.0);
        }

        // Handle timer countdown
        if self.current_timer > 0 {
        self.current_timer -= 1;
        return Some(self.last_sample);
    }

    // Timer expired - get new sample and reset timer
    self.current_timer = if timer_period > 0 { timer_period as u32 } else { 1 };
    let dmc_output = *self.dmc_output.lock().unwrap();
    self.last_sample = (dmc_output as f32 / 64.0) - 1.0;
    Some(self.last_sample)

        // // DMC output is typically 0-127, convert to roughly -1.0 to 1.0
        // // The actual NES DMC output levels aren't linear, but this is a reasonable approximation
        // self.last_sample = if timer_period > 0 {
        //     (dmc_output as f32 / 64.0) - 1.0
        // } else {
        //     0.0
        // };

        // Some(self.last_sample)
    }
}

pub struct Apu {
    mute: Arc<Mutex<bool>>,

    pulse1_duty: Arc<Mutex<u8>>,
    pulse1_sweep: Mutex<u8>,
    pulse1_timer_low: Mutex<u8>,
    pulse1_timer_high: Mutex<u8>,
    pulse1: Arc<SoundChannel>,
    pulse1_sweep_unit: Arc<Mutex<SweepUnit>>,
    pulse1_timer: Arc<Mutex<u16>>,

    pulse2_duty: Arc<Mutex<u8>>,
    pulse2_sweep: Mutex<u8>,
    pulse2_timer_low: Mutex<u8>,
    pulse2_timer_high: Mutex<u8>,
    pulse2: Arc<SoundChannel>,
    pulse2_sweep_unit: Arc<Mutex<SweepUnit>>,
    pulse2_timer: Arc<Mutex<u16>>,

    triangle_linear: Mutex<u8>,
    triangle_timer_low: Mutex<u8>,
    triangle_timer_high: Mutex<u8>,
    triangle: Arc<SoundChannel>,
    triangle_timer: Arc<Mutex<u16>>,

    noise_volume: Mutex<u8>,
    noise_period: Mutex<u8>,
    noise_length: Mutex<u8>,
    noise: Arc<SoundChannel>,
    noise_mode: Arc<Mutex<bool>>,

    // DMC Specific fields
    dmc: Arc<SoundChannel>,
    dmc_control: Mutex<u8>,            // $4010
    dmc_direct_load: Mutex<u8>,        // $4011
    dmc_sample_address_reg: Mutex<u8>, // $4012
    dmc_sample_length_reg: Mutex<u8>,  // $4013

    dmc_current_address: Mutex<u16>,
    dmc_bytes_remaining: Mutex<u16>,
    dmc_sample_buffer: Mutex<u8>,
    dmc_bits_remaining: Mutex<u8>,
    dmc_delta_counter: Arc<Mutex<u8>>,
    dmc_timer: Arc<Mutex<u16>>,    // Timer period for DMC
    dmc_timer_counter: Mutex<u16>, // Current timer value for DMC
    dmc_interrupt_flag: Mutex<bool>,
    dmc_loop_flag: Mutex<bool>,
    dmc_silence: Mutex<bool>,

    status: Mutex<u8>,
    frame_counter: Mutex<u8>,

    audio_thread: Mutex<Option<thread::JoinHandle<()>>>,
    frame_sequencer_thread: Mutex<Option<thread::JoinHandle<()>>>,

    length_counter_table: [u8; 32],
    dmc_period_table: [u16; 16],
    bus: Option<*mut Bus>, // Link to the Bus
    
}

impl Apu {
    pub fn new() -> Self {
        let pulse1 = Arc::new(SoundChannel::new());
        let pulse2 = Arc::new(SoundChannel::new());
        let triangle = Arc::new(SoundChannel::new());
        let noise = Arc::new(SoundChannel::new());
        let dmc = Arc::new(SoundChannel::new()); // Initialize DMC channel

        let pulse1_duty = Arc::new(Mutex::new(0));
        let pulse2_duty = Arc::new(Mutex::new(0));
        let noise_mode = Arc::new(Mutex::new(false));
        let mute = Arc::new(Mutex::new(false));

        let pulse1_sweep_unit =
            Arc::new(Mutex::new(SweepUnit::new(Arc::clone(&pulse1.sweep_mute))));
        let pulse2_sweep_unit =
            Arc::new(Mutex::new(SweepUnit::new(Arc::clone(&pulse2.sweep_mute))));

        let pulse1_timer = Arc::new(Mutex::new(0));
        let pulse2_timer = Arc::new(Mutex::new(0));
        let triangle_timer = Arc::new(Mutex::new(0));
        let dmc_timer = Arc::new(Mutex::new(0)); // Initialize DMC timer
        let dmc_delta_counter = Arc::new(Mutex::new(0)); // Initialize DMC delta counter

        let length_counter_table: [u8; 32] = [
            10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20,
            96, 22, 192, 24, 72, 26, 16, 28, 32, 30,
        ];

        let dmc_period_table: [u16; 16] = [
            428, 380, 340, 320, 286, 254, 226, 214, 190, 160, 142, 128, 106, 85, 71, 64,
        ];

        let apu = Apu {
            mute,
            pulse1_duty,
            pulse1_sweep: Mutex::new(0),
            pulse1_timer_low: Mutex::new(0),
            pulse1_timer_high: Mutex::new(0),
            pulse1,
            pulse1_sweep_unit,
            pulse1_timer,

            pulse2_duty,
            pulse2_sweep: Mutex::new(0),
            pulse2_timer_low: Mutex::new(0),
            pulse2_timer_high: Mutex::new(0),
            pulse2,
            pulse2_sweep_unit,
            pulse2_timer,

            triangle_linear: Mutex::new(0),
            triangle_timer_low: Mutex::new(0),
            triangle_timer_high: Mutex::new(0),
            triangle,
            triangle_timer,

            noise_volume: Mutex::new(0),
            noise_period: Mutex::new(0),
            noise_length: Mutex::new(0),
            noise,
            noise_mode,

            // DMC
            dmc,
            dmc_control: Mutex::new(0),
            dmc_direct_load: Mutex::new(0),
            dmc_sample_address_reg: Mutex::new(0),
            dmc_sample_length_reg: Mutex::new(0),
            dmc_current_address: Mutex::new(0),
            dmc_bytes_remaining: Mutex::new(0),
            dmc_sample_buffer: Mutex::new(0),
            dmc_bits_remaining: Mutex::new(0),
            dmc_delta_counter,
            dmc_timer,
            dmc_timer_counter: Mutex::new(0),
            dmc_interrupt_flag: Mutex::new(false),
            dmc_loop_flag: Mutex::new(false),
            dmc_silence: Mutex::new(true),

            status: Mutex::new(0x0F),
            frame_counter: Mutex::new(0),

            audio_thread: Mutex::new(None),
            length_counter_table,
            dmc_period_table,
            frame_sequencer_thread: Mutex::new(None),
            bus: None,
        };

        apu.start_audio_thread();
        apu.start_frame_sequencer();

        apu
    }

    pub fn link_bus(&mut self, bus: &mut Bus) {
        self.bus = Some(bus);
    }

    pub fn toggle_sound(&mut self) {
        let mute = Arc::clone(&self.mute);
        let b = *mute.lock().unwrap();
        *mute.lock().unwrap() = !b;
    }

    fn start_frame_sequencer(&self) {
        let pulse1 = Arc::clone(&self.pulse1);
        let pulse2 = Arc::clone(&self.pulse2);
        let triangle = Arc::clone(&self.triangle);
        let noise = Arc::clone(&self.noise);
        let pulse1_sweep_unit = Arc::clone(&self.pulse1_sweep_unit);
        let pulse2_sweep_unit = Arc::clone(&self.pulse2_sweep_unit);

        let pulse1_timer_in_thread = Arc::clone(&self.pulse1_timer);
        let pulse2_timer_in_thread = Arc::clone(&self.pulse2_timer);

        let handle = thread::spawn(move || {
            loop {
                // The frame sequencer runs at a fixed rate, often tied to CPU cycles or a specific clock.
                // For simplicity in this example, a fixed sleep duration is used, but in a real emulator,
                // this would be synchronized with the CPU clock.
                thread::sleep(Duration::from_millis(4));

                let current_pulse1_timer = *pulse1_timer_in_thread.lock().unwrap();
                let current_pulse2_timer = *pulse2_timer_in_thread.lock().unwrap();

                // Clock sweep units
                if let Some(new_timer) = pulse1_sweep_unit
                    .lock()
                    .unwrap()
                    .clock(current_pulse1_timer, 1)
                {
                    *pulse1_timer_in_thread.lock().unwrap() = new_timer;
                    pulse1.set_frequency(Apu::get_frequency_from_timer_value(new_timer));
                }

                if let Some(new_timer) = pulse2_sweep_unit
                    .lock()
                    .unwrap()
                    .clock(current_pulse2_timer, 2)
                {
                    *pulse2_timer_in_thread.lock().unwrap() = new_timer;
                    pulse2.set_frequency(Apu::get_frequency_from_timer_value(new_timer));
                }

                // Decrement length counters
                let pulse1_active_due_to_length = pulse1.decrement_length_counter();
                let pulse2_active_due_to_length = pulse2.decrement_length_counter();
                let triangle_active_due_to_length = triangle.decrement_length_counter();
                let noise_active_due_to_length = noise.decrement_length_counter();

                // Determine final channel enabled state (considering sweep mute for pulse channels)
                let pulse1_final_enabled =
                    pulse1_active_due_to_length && !*pulse1.sweep_mute.lock().unwrap();
                let pulse2_final_enabled =
                    pulse2_active_due_to_length && !*pulse2.sweep_mute.lock().unwrap();

                pulse1.set_enabled(pulse1_final_enabled);
                pulse2.set_enabled(pulse2_final_enabled);
                triangle.set_enabled(triangle_active_due_to_length);
                noise.set_enabled(noise_active_due_to_length);
            }
        });

        *self.frame_sequencer_thread.lock().unwrap() = Some(handle);
    }

    fn start_audio_thread(&self) {
        let pulse1 = Arc::clone(&self.pulse1);
        let pulse2 = Arc::clone(&self.pulse2);
        let triangle = Arc::clone(&self.triangle);
        let noise = Arc::clone(&self.noise);
        let dmc = Arc::clone(&self.dmc); // Clone for DMC
        let dmc_output = Arc::clone(&self.dmc_delta_counter); // For DMC output
        let dmc_timer_period = Arc::clone(&self.dmc_timer); // For DMC timer period

        let pulse1_duty = Arc::clone(&self.pulse1_duty);
        let pulse2_duty = Arc::clone(&self.pulse2_duty);
        let noise_mode = Arc::clone(&self.noise_mode);
        let mute = Arc::clone(&self.mute);

        let handle = thread::spawn(move || {
            let (_stream, stream_handle) = match OutputStream::try_default() {
                Ok(result) => result,
                Err(_) => {
                    eprintln!("Error initializing audio stream: Could not open output stream.");
                    return;
                }
            };

            let sink = match Sink::try_new(&stream_handle) {
                Ok(sink) => sink,
                Err(_) => {
                    eprintln!("Error initializing audio sink: Could not create sink.");
                    return;
                }
            };

            let source1 = PulseWaveSource::new(Arc::clone(&pulse1), Arc::clone(&pulse1_duty))
                .convert_samples::<f32>();

            let source2 = PulseWaveSource::new(Arc::clone(&pulse2), Arc::clone(&pulse2_duty))
                .convert_samples::<f32>();

            let source3 = TriangleWaveSource::new(Arc::clone(&triangle)).convert_samples::<f32>();

            let source4 = NoiseSource::new(Arc::clone(&noise), Arc::clone(&noise_mode))
                .convert_samples::<f32>();

            let source5 = DmcSource::new(Arc::clone(&dmc), dmc_output, dmc_timer_period)
                .convert_samples::<f32>();

            let mixed_pulse = source1.mix(source2);
            let mixed_pulse_triangle = mixed_pulse.mix(source3);
            let mixed_all = mixed_pulse_triangle.mix(source4);
            let mixed_all_dmc = mixed_all.mix(source5); // Mix DMC

            let final_source = mixed_all_dmc.amplify(0.5); // Amplify all mixed channels

            sink.append(final_source);
            sink.play();

            loop {
                thread::sleep(Duration::from_millis(100));

                let is_muted = *mute.lock().unwrap();
                if is_muted {
                    sink.pause();
                } else {
                    sink.play();
                }
            }
        });

        *self.audio_thread.lock().unwrap() = Some(handle);
    }

    // Helper to read from Bus (CPU memory)
    fn cpu_read_bus(&self, address: u16) -> u8 {
        unsafe {
            if let Some(bus_ptr) = self.bus {
                (*bus_ptr).cpu_read(address, false)
            } else {
                // This should not happen if bus is properly linked
                eprintln!("APU: Bus not linked for CPU read at address {:X}", address);
                0 // Return 0 or appropriate default
            }
        }
    }

    // DMC internal clocking and sample fetching
    pub fn dmc_clock(&mut self) {
        if *self.dmc_timer_counter.lock().unwrap() > 0 {
            *self.dmc_timer_counter.lock().unwrap() -= 1;
        }

        if *self.dmc_timer_counter.lock().unwrap() == 0 {
            // Reset timer
            *self.dmc_timer_counter.lock().unwrap() = *self.dmc_timer.lock().unwrap();

            if !*self.dmc_silence.lock().unwrap() {
                // Apply delta
                // Corrected bit extraction: read MSB first
                let sample_bit = (*self.dmc_sample_buffer.lock().unwrap()
                    >> *self.dmc_bits_remaining.lock().unwrap())
                    & 0x01;
                let mut delta_counter = *self.dmc_delta_counter.lock().unwrap();

                if sample_bit == 1 {
                    if delta_counter <= 125 {
                        delta_counter += 2;
                    }
                } else {
                    // sample_bit == 0
                    if delta_counter >= 2 {
                        delta_counter -= 2;
                    }
                }
                *self.dmc_delta_counter.lock().unwrap() = delta_counter;
            }

            if *self.dmc_bits_remaining.lock().unwrap() == 0 {
                // Fetch next byte
                self.dmc_fetch_sample();
            } else {
                *self.dmc_bits_remaining.lock().unwrap() -= 1;
            }
        }
    }

    fn dmc_fetch_sample(&mut self) {
        if *self.dmc_bytes_remaining.lock().unwrap() > 0 {
            // Read from CPU memory
            *self.dmc_sample_buffer.lock().unwrap() =
                self.cpu_read_bus(*self.dmc_current_address.lock().unwrap());
            *self.dmc_silence.lock().unwrap() = false;
            *self.dmc_bits_remaining.lock().unwrap() = 7; // 8 bits in buffer, 0-7, so 7 remaining after first is read

            // Increment address, wrap if necessary
            // According to NESDev, DMC addresses wrap from 0xFFFF to 0x8000
            if *self.dmc_current_address.lock().unwrap() == 0xFFFF {
                *self.dmc_current_address.lock().unwrap() = 0x8000;
            } else {
                *self.dmc_current_address.lock().unwrap() += 1;
            }

            *self.dmc_bytes_remaining.lock().unwrap() -= 1;

            if *self.dmc_bytes_remaining.lock().unwrap() == 0 {
                // End of sample
                if *self.dmc_loop_flag.lock().unwrap() {
                    self.dmc_start_playback(); // Loop
                } else {
                    *self.dmc_interrupt_flag.lock().unwrap() = true; // Set IRQ
                }
            }
        } else {
            *self.dmc_silence.lock().unwrap() = true;
        }
    }

    fn dmc_start_playback(&mut self) {
        *self.dmc_current_address.lock().unwrap() =
            0xC000 | ((*self.dmc_sample_address_reg.lock().unwrap() as u16) << 6);
        *self.dmc_bytes_remaining.lock().unwrap() =
            ((*self.dmc_sample_length_reg.lock().unwrap() as u16) << 4) + 1;
        *self.dmc_timer_counter.lock().unwrap() = 0; // Reset timer counter to trigger immediate fetch
        *self.dmc_bits_remaining.lock().unwrap() = 0; // Reset bits remaining to trigger immediate fetch
        self.dmc.set_enabled(true);
        *self.dmc_silence.lock().unwrap() = false; // Not silent anymore
        *self.dmc_interrupt_flag.lock().unwrap() = false; // Clear IRQ
    }

    pub fn cpu_write(&mut self, address: u16, data: u8) {
        match address {
            0x4000 => {
                *self.pulse1_duty.lock().unwrap() = data;

                let volume = (data & 0x0F) as f32 / 15.0;
                self.pulse1.set_volume(volume);

                self.pulse1.set_length_counter_halt((data & 0x20) != 0);

                let status = *self.status.lock().unwrap();
                self.pulse1.set_enabled((status & 0x01) != 0);
            }
            0x4001 => {
                *self.pulse1_sweep.lock().unwrap() = data;
                self.pulse1_sweep_unit.lock().unwrap().write(data);
            }
            0x4002 => {
                *self.pulse1_timer_low.lock().unwrap() = data;
                self.update_pulse1_frequency();
            }
            0x4003 => {
                *self.pulse1_timer_high.lock().unwrap() = data & 0b0000_0111;
                self.update_pulse1_frequency();

                let length_index = (data >> 3) & 0x1F;
                let length_value = self.length_counter_table[length_index as usize];
                self.pulse1.set_length_counter(length_value);

                self.pulse1_sweep_unit.lock().unwrap().reset_divider();

                let status = *self.status.lock().unwrap();
                if (status & 0x01) != 0 {
                    self.pulse1.set_enabled(true);
                }
            }

            0x4004 => {
                *self.pulse2_duty.lock().unwrap() = data;

                let volume = (data & 0x0F) as f32 / 15.0;
                self.pulse2.set_volume(volume);

                self.pulse2.set_length_counter_halt((data & 0x20) != 0);

                let status = *self.status.lock().unwrap();
                self.pulse2.set_enabled((status & 0x02) != 0);
            }
            0x4005 => {
                *self.pulse2_sweep.lock().unwrap() = data;
                self.pulse2_sweep_unit.lock().unwrap().write(data);
            }
            0x4006 => {
                *self.pulse2_timer_low.lock().unwrap() = data;
                self.update_pulse2_frequency();
            }
            0x4007 => {
                *self.pulse2_timer_high.lock().unwrap() = data & 0b0000_0111;
                self.update_pulse2_frequency();

                let length_index = (data >> 3) & 0x1F;
                let length_value = self.length_counter_table[length_index as usize];
                self.pulse2.set_length_counter(length_value);

                self.pulse2_sweep_unit.lock().unwrap().reset_divider();

                let status = *self.status.lock().unwrap();
                if (status & 0x02) != 0 {
                    self.pulse2.set_enabled(true);
                }
            }

            0x4008 => {
                *self.triangle_linear.lock().unwrap() = data;
                self.triangle.set_length_counter_halt((data & 0x80) != 0);
                let status = *self.status.lock().unwrap();
                self.triangle.set_enabled((status & 0x04) != 0);
                self.triangle.set_volume(0.9);
            }
            0x4009 => {}
            0x400A => {
                *self.triangle_timer_low.lock().unwrap() = data;
                self.update_triangle_frequency();
            }
            0x400B => {
                *self.triangle_timer_high.lock().unwrap() = data & 0b0000_0111;
                self.update_triangle_frequency();

                let length_index = (data >> 3) & 0x1F;
                let length_value = self.length_counter_table[length_index as usize];
                self.triangle.set_length_counter(length_value);

                let status = *self.status.lock().unwrap();
                if (status & 0x04) != 0 {
                    self.triangle.set_enabled(true);
                }
            }

            0x400C => {
                *self.noise_volume.lock().unwrap() = data;

                let volume = (data & 0x0F) as f32 / 15.0;
                self.noise.set_volume(volume);

                self.noise.set_length_counter_halt((data & 0x20) != 0);

                let status = *self.status.lock().unwrap();
                self.noise.set_enabled((status & 0x08) != 0);
            }
            0x400D => {}
            0x400E => {
                *self.noise_period.lock().unwrap() = data;

                *self.noise_mode.lock().unwrap() = (data & 0x80) != 0;

                self.update_noise_frequency();
            }
            0x400F => {
                *self.noise_length.lock().unwrap() = data;

                let length_index = (data >> 3) & 0x1F;
                let length_value = self.length_counter_table[length_index as usize];
                self.noise.set_length_counter(length_value);

                let status = *self.status.lock().unwrap();
                if (status & 0x08) != 0 {
                    self.noise.set_enabled(true);
                }
            }
            // DMC Registers
            0x4010 => {
                // DMC_FREQ / CONTROL
                *self.dmc_control.lock().unwrap() = data;
                *self.dmc_loop_flag.lock().unwrap() = (data & 0x40) != 0;
                // Corrected: Clear IRQ flag if IRQ enable bit (0x80) is 0
                if (data & 0x80) == 0 {
                    *self.dmc_interrupt_flag.lock().unwrap() = false;
                } else {
                    // If IRQ enable bit is 1, the IRQ flag is set if bytes remaining is 0
                    // This is handled by dmc_fetch_sample
                    // No change needed here for setting the flag to true based on this bit,
                    // as it only enables/disables the IRQ generation.
                }
                *self.dmc_timer.lock().unwrap() = self.dmc_period_table[(data & 0x0F) as usize];
            }
            0x4011 => {
                // DMC_DAC
                *self.dmc_direct_load.lock().unwrap() = data;
                *self.dmc_delta_counter.lock().unwrap() = data & 0x7F; // 7-bit value
            }
            0x4012 => {
                // DMC_ADDR
                *self.dmc_sample_address_reg.lock().unwrap() = data;
                // Actual address is $C000 + (value * 64)
            }
            0x4013 => {
                // DMC_LEN
                *self.dmc_sample_length_reg.lock().unwrap() = data;
                // Actual length is (value * 16) + 1 bytes
            }

            0x4015 => {
                *self.status.lock().unwrap() = data & 0x0F;

                if (data & 0x01) == 0 {
                    self.pulse1.set_length_counter(0);
                }
                self.pulse1.set_enabled((data & 0x01) != 0);

                if (data & 0x02) == 0 {
                    self.pulse2.set_length_counter(0);
                }
                self.pulse2.set_enabled((data & 0x02) != 0);

                if (data & 0x04) == 0 {
                    self.triangle.set_length_counter(0);
                }
                self.triangle.set_enabled((data & 0x04) != 0);

                if (data & 0x08) == 0 {
                    self.noise.set_length_counter(0);
                }
                self.noise.set_enabled((data & 0x08) != 0);

                // DMC enable/disable/restart
                if (data & 0x10) != 0 {
                    // DMC enabled
                    if *self.dmc_bytes_remaining.lock().unwrap() == 0 {
                        self.dmc_start_playback();
                    }
                } else {
                    // DMC disabled
                    *self.dmc_bytes_remaining.lock().unwrap() = 0;
                    self.dmc.set_enabled(false);
                }
                *self.dmc_interrupt_flag.lock().unwrap() = false; // Clear interrupt flag on write to $4015
            }

            0x4017 => {
                *self.frame_counter.lock().unwrap() = data;
            }

            _ => {}
        }
    }

    pub fn cpu_read(&self, address: u16) -> u8 {
        match address {
            0x4000 => *self.pulse1_duty.lock().unwrap(),
            0x4001 => *self.pulse1_sweep.lock().unwrap(),
            0x4002 => *self.pulse1_timer_low.lock().unwrap(),
            0x4003 => *self.pulse1_timer_high.lock().unwrap(),
            0x4004 => *self.pulse2_duty.lock().unwrap(),
            0x4005 => *self.pulse2_sweep.lock().unwrap(),
            0x4006 => *self.pulse2_timer_low.lock().unwrap(),
            0x4007 => *self.pulse2_timer_high.lock().unwrap(),
            0x4008 => *self.triangle_linear.lock().unwrap(),
            0x4009 => 0,
            0x400A => *self.triangle_timer_low.lock().unwrap(),
            0x400B => *self.triangle_timer_high.lock().unwrap(),
            0x400C => *self.noise_volume.lock().unwrap(),
            0x400D => 0,
            0x400E => *self.noise_period.lock().unwrap(),
            0x400F => *self.noise_length.lock().unwrap(),

            0x4015 => {
                let mut status = 0x00;

                if *self.pulse1.length_counter.lock().unwrap() > 0 {
                    status |= 0x01;
                }

                if *self.pulse2.length_counter.lock().unwrap() > 0 {
                    status |= 0x02;
                }

                if *self.triangle.length_counter.lock().unwrap() > 0 {
                    status |= 0x04;
                }

                if *self.noise.length_counter.lock().unwrap() > 0 {
                    status |= 0x08;
                }

                // DMC status
                if *self.dmc_bytes_remaining.lock().unwrap() > 0 {
                    status |= 0x10;
                }
                if *self.dmc_interrupt_flag.lock().unwrap() {
                    status |= 0x80;
                }
                *self.dmc_interrupt_flag.lock().unwrap() = false; // Clear interrupt flag on read

                status
            }

            0x4017 => *self.frame_counter.lock().unwrap(),

            _ => 0,
        }
    }

    fn get_frequency_from_timer_value(timer: u16) -> f32 {
        if timer == 0 {
            0.0
        } else {
            1_789_773.0 / (16.0 * (timer as f32 + 1.0))
        }
    }

    fn update_pulse1_frequency(&self) {
        let high = *self.pulse1_timer_high.lock().unwrap();
        let low = *self.pulse1_timer_low.lock().unwrap();
        let timer = ((high as u16 & 0x07) << 8) | low as u16;
        *self.pulse1_timer.lock().unwrap() = timer;
        let freq = Self::get_frequency_from_timer_value(timer);
        self.pulse1.set_frequency(freq);
    }

    fn update_pulse2_frequency(&self) {
        let high = *self.pulse2_timer_high.lock().unwrap();
        let low = *self.pulse2_timer_low.lock().unwrap();
        let timer = ((high as u16 & 0x07) << 8) | low as u16;
        *self.pulse2_timer.lock().unwrap() = timer;
        let freq = Self::get_frequency_from_timer_value(timer);
        self.pulse2.set_frequency(freq);
    }

    fn update_triangle_frequency(&self) {
        let high = *self.triangle_timer_high.lock().unwrap();
        let low = *self.triangle_timer_low.lock().unwrap();
        let timer = ((high as u16 & 0x07) << 8) | low as u16;
        *self.triangle_timer.lock().unwrap() = timer;

        let freq = if timer == 0 {
            0.0
        } else {
            1_789_773.0 / (32.0 * (timer as f32 + 1.0))
        };
        self.triangle.set_frequency(freq);
    }

    fn update_noise_frequency(&self) {
        let period_idx = *self.noise_period.lock().unwrap() & 0x0F;

        let period = PERIOD_TABLE[period_idx as usize];

        let freq = 1_789_773.0 / (period as f32 * 2.0);
        self.noise.set_frequency(freq);
    }
}

impl Drop for Apu {
    fn drop(&mut self) {
        self.pulse1.set_enabled(false);
        self.pulse2.set_enabled(false);
        self.triangle.set_enabled(false);
        self.noise.set_enabled(false);
        self.dmc.set_enabled(false); // Disable DMC on drop
    }
}
