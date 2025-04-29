use rodio::{OutputStream, Sink, Source};
use std::sync::{Arc, Mutex}; // Thread-safe shared ownership and mutability
use std::thread; // For spawning separate threads
use std::time::Duration; // For timing operations // Audio library for output

/// Represents a generic sound channel in the NES APU
/// Manages the state of an audio channel including frequency, volume, and length counter
struct SoundChannel {
    // Core sound parameters
    frequency: Arc<Mutex<f32>>, // The frequency of the sound in Hz, thread-safe
    volume: Arc<Mutex<f32>>,    // The volume level from 0.0 to 1.0, thread-safe
    enabled: Arc<Mutex<bool>>,  // Whether the channel is currently active, thread-safe

    // NES-specific length counter system
    length_counter: Arc<Mutex<u8>>, // The length counter value (determines how long the sound plays)
    length_counter_enabled: Arc<Mutex<bool>>, // Whether the length counter system is active
    length_counter_halt: Arc<Mutex<bool>>, // Whether the length counter is prevented from decrementing
}

impl SoundChannel {
    /// Creates a new sound channel with default values
    fn new() -> Self {
        SoundChannel {
            frequency: Arc::new(Mutex::new(0.0)),    // Start at 0Hz (silent)
            volume: Arc::new(Mutex::new(0.0)),       // Start at volume 0 (silent)
            enabled: Arc::new(Mutex::new(false)),    // Start disabled
            length_counter: Arc::new(Mutex::new(0)), // Start with counter at 0
            length_counter_enabled: Arc::new(Mutex::new(true)), // Length counter enabled by default
            length_counter_halt: Arc::new(Mutex::new(false)), // Length counter not halted by default
        }
    }

    /// Sets the frequency of this sound channel
    fn set_frequency(&self, freq: f32) {
        *self.frequency.lock().unwrap() = freq; // Lock the mutex and update the value
    }

    /// Sets the volume of this sound channel (0.0 to 1.0)
    fn set_volume(&self, vol: f32) {
        *self.volume.lock().unwrap() = vol; // Lock the mutex and update the value
    }

    /// Enables or disables this sound channel
    /// If disabled, also resets the length counter to 0
    fn set_enabled(&self, enabled: bool) {
        *self.enabled.lock().unwrap() = enabled; // Lock the mutex and update the value

        // If disabling the channel, reset the length counter
        if !enabled {
            *self.length_counter.lock().unwrap() = 0;
        }
    }

    /// Sets the length counter value for timed sounds
    fn set_length_counter(&self, value: u8) {
        *self.length_counter.lock().unwrap() = value; // Lock the mutex and update the value
    }

    /// Sets whether the length counter decrement should be halted
    fn set_length_counter_halt(&self, halt: bool) {
        *self.length_counter_halt.lock().unwrap() = halt; // Lock the mutex and update the value
    }

    /// Decrements the length counter if appropriate
    /// Returns true if the channel should continue playing, false if it should stop
    fn decrement_length_counter(&self) -> bool {
        let mut counter = self.length_counter.lock().unwrap(); // Lock the counter for modification
        let halt = *self.length_counter_halt.lock().unwrap(); // Check if counter is halted
        let enabled = *self.length_counter_enabled.lock().unwrap(); // Check if counter is enabled

        // Don't decrement if halted or counter system is disabled
        if halt || !enabled {
            return *counter > 0; // Keep playing if counter is still above 0
        }

        // Decrement if counter is positive
        if *counter > 0 {
            *counter -= 1;
            return *counter > 0; // Keep playing if counter is still above 0 after decrement
        }

        false // Stop playing if counter is 0
    }
}

/// Implements a pulse wave (square wave with variable duty cycle)
/// Used for the two pulse channels in the NES
struct PulseWaveSource {
    channel: Arc<SoundChannel>, // The sound channel containing the parameters
    duty: Arc<Mutex<u8>>,       // The duty cycle register value (controls waveform shape)
    sample_rate: u32,           // The audio sample rate (usually 44100 Hz)
    position: f32,              // Current position in the waveform cycle (0.0 to 1.0)
}

impl PulseWaveSource {
    /// Creates a new pulse wave source with the given channel and duty cycle
    fn new(channel: Arc<SoundChannel>, duty: Arc<Mutex<u8>>) -> Self {
        PulseWaveSource {
            channel,
            duty,
            sample_rate: 44100, // Standard CD-quality sample rate
            position: 0.0,      // Start at beginning of waveform
        }
    }

    /// Calculates the current output value based on position in the waveform
    /// and the current duty cycle setting
    fn get_duty_output(&self, position: f32) -> f32 {
        // Extract the duty cycle bits (bits 6-7) from the duty register
        // and convert to an actual duty cycle percentage
        let duty_cycle = match *self.duty.lock().unwrap() & 0xC0 {
            0x00 => 0.125, // 12.5% - shortest pulse
            0x40 => 0.25,  // 25%
            0x80 => 0.5,   // 50% - standard square wave
            0xC0 => 0.75,  // 75% - inverted 25% pulse
            _ => 0.5,      // Fallback (shouldn't happen)
        };

        // Generate +1.0 for the "on" portion of the duty cycle
        // and -1.0 for the "off" portion
        if position < duty_cycle {
            1.0 // "On" portion
        } else {
            -1.0 // "Off" portion
        }
    }
}

// Implementation of rodio's Source trait for PulseWaveSource
// This allows it to be used with the audio library
impl Source for PulseWaveSource {
    // Maximum number of frames that can be generated at once
    fn current_frame_len(&self) -> Option<usize> {
        None // No limit on frames
    }

    // Number of audio channels (mono)
    fn channels(&self) -> u16 {
        1 // Mono output
    }

    // Audio sample rate
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    // Total duration of the sound
    fn total_duration(&self) -> Option<Duration> {
        None // Continuous sound with no fixed duration
    }
}

// Implementation of the Iterator trait for PulseWaveSource
// This generates the actual audio samples one by one
impl Iterator for PulseWaveSource {
    type Item = f32; // Each sample is a 32-bit float

    fn next(&mut self) -> Option<f32> {
        // Get the current parameters from the channel
        let freq = *self.channel.frequency.lock().unwrap();
        let vol = *self.channel.volume.lock().unwrap();
        let enabled = *self.channel.enabled.lock().unwrap();
        let length_counter = *self.channel.length_counter.lock().unwrap();

        // Return silence if the channel is disabled or frequency is 0
        // or length counter has expired
        if !enabled || freq <= 0.0 || length_counter == 0 {
            self.position = 0.0;
            return Some(0.0); // Silent sample
        }

        // Get the current waveform value and apply volume
        let sample = self.get_duty_output(self.position) * vol;

        // Advance the position in the waveform based on frequency
        self.position += freq / self.sample_rate as f32;

        // Wrap around if we've completed a cycle
        while self.position >= 1.0 {
            self.position -= 1.0;
        }

        Some(sample) // Return the calculated sample
    }
}

/// Implements a triangle wave generator
/// The triangle channel in the NES has a fixed volume but variable frequency
struct TriangleWaveSource {
    channel: Arc<SoundChannel>, // The sound channel containing the parameters
    sample_rate: u32,           // The audio sample rate (usually 44100 Hz)
    position: f32,              // Current position in the waveform cycle (0.0 to 1.0)
}

impl TriangleWaveSource {
    /// Creates a new triangle wave source with the given channel
    fn new(channel: Arc<SoundChannel>) -> Self {
        TriangleWaveSource {
            channel,
            sample_rate: 44100, // Standard CD-quality sample rate
            position: 0.0,      // Start at beginning of waveform
        }
    }
}

// Implementation of rodio's Source trait for TriangleWaveSource
// This allows it to be used with the audio library
impl Source for TriangleWaveSource {
    // Maximum number of frames that can be generated at once
    fn current_frame_len(&self) -> Option<usize> {
        None // No limit on frames
    }

    // Number of audio channels (mono)
    fn channels(&self) -> u16 {
        1 // Mono output
    }

    // Audio sample rate
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    // Total duration of the sound
    fn total_duration(&self) -> Option<Duration> {
        None // Continuous sound with no fixed duration
    }
}

// Implementation of the Iterator trait for TriangleWaveSource
// This generates the actual audio samples one by one
impl Iterator for TriangleWaveSource {
    type Item = f32; // Each sample is a 32-bit float

    fn next(&mut self) -> Option<f32> {
        // Get the current parameters from the channel
        let freq = *self.channel.frequency.lock().unwrap();
        let vol = *self.channel.volume.lock().unwrap();
        let enabled = *self.channel.enabled.lock().unwrap();
        let length_counter = *self.channel.length_counter.lock().unwrap();

        // Return silence if the channel is disabled or frequency is 0
        // or length counter has expired
        if !enabled || freq <= 0.0 || length_counter == 0 {
            self.position = 0.0;
            return Some(0.0); // Silent sample
        }

        // Generate triangular waveform
        // This creates a linear ramp up and down pattern
        let sample = if self.position < 0.5 {
            (self.position * 4.0 - 1.0) * vol // Rising slope from -1 to +1
        } else {
            (3.0 - self.position * 4.0) * vol // Falling slope from +1 to -1
        };

        // Advance the position in the waveform based on frequency
        self.position += freq / self.sample_rate as f32;

        // Wrap around if we've completed a cycle
        while self.position >= 1.0 {
            self.position -= 1.0;
        }

        Some(sample) // Return the calculated sample
    }
}

/// Implements a noise generator using a linear feedback shift register
/// Used for the noise channel in the NES
struct NoiseSource {
    channel: Arc<SoundChannel>,  // The sound channel containing the parameters
    mode_flag: Arc<Mutex<bool>>, // The noise mode flag (affects feedback pattern)
    sample_rate: u32,            // The audio sample rate (usually 44100 Hz)
    shift_register: u16,         // The 15-bit shift register used to generate pseudorandom noise
    sample_period: f32,          // Time between shift register updates
    sample_timer: f32,           // Timer for tracking when to update shift register
    current_output: f32,         // Current output value (-1.0 or 1.0)
}

impl NoiseSource {
    /// Creates a new noise source with the given channel and mode flag
    fn new(channel: Arc<SoundChannel>, mode_flag: Arc<Mutex<bool>>) -> Self {
        NoiseSource {
            channel,
            mode_flag,
            sample_rate: 44100,  // Standard CD-quality sample rate
            shift_register: 1,   // Initial shift register value (non-zero)
            sample_period: 0.0,  // Will be calculated based on frequency
            sample_timer: 0.0,   // Start with timer at 0
            current_output: 0.0, // Start with zero output
        }
    }

    /// Updates the shift register to generate the next pseudorandom value
    /// The method calculates a feedback bit based on XOR operations
    /// and shifts the register accordingly
    fn update_shift_register(&mut self) {
        // Get the lowest bit of the shift register
        let bit0 = self.shift_register & 1;

        // Calculate feedback bit using different taps based on mode
        // Mode 0: XOR bits 0 and 1
        // Mode 1: XOR bits 0 and 6
        let feedback_bit = if *self.mode_flag.lock().unwrap() {
            ((self.shift_register >> 6) & 1) ^ bit0
        } else {
            ((self.shift_register >> 1) & 1) ^ bit0
        };

        // Shift the register right by 1 bit
        self.shift_register >>= 1;

        // Insert the feedback bit at the top (bit 14)
        self.shift_register |= feedback_bit << 14;

        // Set the output based on bit 0
        // This creates a "random" pattern of -1.0 and 1.0 values
        self.current_output = if bit0 == 0 { 1.0 } else { -1.0 };
    }
}

// Implementation of rodio's Source trait for NoiseSource
// This allows it to be used with the audio library
impl Source for NoiseSource {
    // Maximum number of frames that can be generated at once
    fn current_frame_len(&self) -> Option<usize> {
        None // No limit on frames
    }

    // Number of audio channels (mono)
    fn channels(&self) -> u16 {
        1 // Mono output
    }

    // Audio sample rate
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    // Total duration of the sound
    fn total_duration(&self) -> Option<Duration> {
        None // Continuous sound with no fixed duration
    }
}

// Implementation of the Iterator trait for NoiseSource
// This generates the actual audio samples one by one
impl Iterator for NoiseSource {
    type Item = f32; // Each sample is a 32-bit float

    fn next(&mut self) -> Option<f32> {
        // Get the current parameters from the channel
        let freq = *self.channel.frequency.lock().unwrap();
        let vol = *self.channel.volume.lock().unwrap();
        let enabled = *self.channel.enabled.lock().unwrap();
        let length_counter = *self.channel.length_counter.lock().unwrap();

        // Return silence if the channel is disabled or frequency is 0
        // or length counter has expired
        if !enabled || freq <= 0.0 || length_counter == 0 {
            return Some(0.0); // Silent sample
        }

        // Increment the timer
        self.sample_timer += 1.0 / self.sample_rate as f32;

        // Calculate how long to wait between shift register updates
        // based on the frequency
        self.sample_period = if freq > 0.0 { 1.0 / freq } else { 0.0 };

        // Update the shift register when the timer expires
        if self.sample_period > 0.0 && self.sample_timer >= self.sample_period {
            self.sample_timer = 0.0; // Reset the timer
            self.update_shift_register(); // Generate next noise value
        }

        // Return the current output value with volume applied
        Some(self.current_output * vol)
    }
}

/// The main Audio Processing Unit (APU) for NES emulation
/// Manages all sound channels and provides register access
pub struct Apu {
    // Pulse 1 channel registers and state
    pulse1_duty: Arc<Mutex<u8>>, // $4000: Duty cycle, envelope, and length counter halt
    pulse1_sweep: Mutex<u8>,     // $4001: Sweep control
    pulse1_timer_low: Mutex<u8>, // $4002: Timer low byte
    pulse1_timer_high: Mutex<u8>, // $4003: Timer high byte and length counter
    pulse1: Arc<SoundChannel>,   // The actual sound channel

    // Pulse 2 channel registers and state
    pulse2_duty: Arc<Mutex<u8>>, // $4004: Duty cycle, envelope, and length counter halt
    pulse2_sweep: Mutex<u8>,     // $4005: Sweep control
    pulse2_timer_low: Mutex<u8>, // $4006: Timer low byte
    pulse2_timer_high: Mutex<u8>, // $4007: Timer high byte and length counter
    pulse2: Arc<SoundChannel>,   // The actual sound channel

    // Triangle channel registers and state
    triangle_linear: Mutex<u8>, // $4008: Linear counter and length counter halt
    triangle_timer_low: Mutex<u8>, // $400A: Timer low byte
    triangle_timer_high: Mutex<u8>, // $400B: Timer high byte and length counter
    triangle: Arc<SoundChannel>, // The actual sound channel

    // Noise channel registers and state
    noise_volume: Mutex<u8>,      // $400C: Volume and envelope
    noise_period: Mutex<u8>,      // $400E: Noise period and mode
    noise_length: Mutex<u8>,      // $400F: Length counter value
    noise: Arc<SoundChannel>,     // The actual sound channel
    noise_mode: Arc<Mutex<bool>>, // Mode flag for noise generation

    // APU control registers
    status: Mutex<u8>,        // $4015: Channel enable flags
    frame_counter: Mutex<u8>, // $4017: Frame counter control

    // Thread management
    audio_thread: Mutex<Option<thread::JoinHandle<()>>>, // Handle for the audio output thread

    // Length counter lookup table
    length_counter_table: [u8; 32], // Maps register values to duration values

    // Frame sequencer thread for timing
    frame_sequencer_thread: Mutex<Option<thread::JoinHandle<()>>>, // Handle for frame sequencer thread
}

impl Apu {
    /// Creates a new APU instance with all channels initialized
    pub fn new() -> Self {
        // Create the sound channels
        let pulse1 = Arc::new(SoundChannel::new());
        let pulse2 = Arc::new(SoundChannel::new());
        let triangle = Arc::new(SoundChannel::new());
        let noise = Arc::new(SoundChannel::new());

        // Create shared register values
        let pulse1_duty = Arc::new(Mutex::new(0));
        let pulse2_duty = Arc::new(Mutex::new(0));
        let noise_mode = Arc::new(Mutex::new(false));

        // Length counter lookup table (used by NES hardware)
        // Converts register values to actual counter values
        let length_counter_table: [u8; 32] = [
            10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20,
            96, 22, 192, 24, 72, 26, 16, 28, 32, 30,
        ];

        // Create the APU instance
        let apu = Apu {
            pulse1_duty,
            pulse1_sweep: Mutex::new(0),
            pulse1_timer_low: Mutex::new(0),
            pulse1_timer_high: Mutex::new(0),
            pulse1,

            pulse2_duty,
            pulse2_sweep: Mutex::new(0),
            pulse2_timer_low: Mutex::new(0),
            pulse2_timer_high: Mutex::new(0),
            pulse2,

            triangle_linear: Mutex::new(0),
            triangle_timer_low: Mutex::new(0),
            triangle_timer_high: Mutex::new(0),
            triangle,

            noise_volume: Mutex::new(0),
            noise_period: Mutex::new(0),
            noise_length: Mutex::new(0),
            noise,
            noise_mode,

            status: Mutex::new(0x0F), // All channels enabled by default
            frame_counter: Mutex::new(0),

            audio_thread: Mutex::new(None),
            length_counter_table,
            frame_sequencer_thread: Mutex::new(None),
        };

        // Start the audio output thread
        apu.start_audio_thread();

        // Start the frame sequencer for timing updates
        apu.start_frame_sequencer();

        apu
    }

    /// Starts the frame sequencer thread
    /// This periodically decrements length counters to control sound duration
    fn start_frame_sequencer(&self) {
        // Clone references to channels for use in thread
        let pulse1 = Arc::clone(&self.pulse1);
        let pulse2 = Arc::clone(&self.pulse2);
        let triangle = Arc::clone(&self.triangle);
        let noise = Arc::clone(&self.noise);

        // Create a new thread for the frame sequencer
        let handle = thread::spawn(move || {
            loop {
                // Sleep for 4ms (roughly the NES frame sequencer rate)
                thread::sleep(Duration::from_millis(4));

                // Update each channel's length counter
                // If it returns false, disable the channel
                if !pulse1.decrement_length_counter() {
                    pulse1.set_enabled(false);
                }

                if !pulse2.decrement_length_counter() {
                    pulse2.set_enabled(false);
                }

                if !triangle.decrement_length_counter() {
                    triangle.set_enabled(false);
                }

                if !noise.decrement_length_counter() {
                    noise.set_enabled(false);
                }
            }
        });

        // Store the thread handle
        *self.frame_sequencer_thread.lock().unwrap() = Some(handle);
    }

    /// Starts the audio output thread
    /// This mixes all channels and sends them to the audio output device
    fn start_audio_thread(&self) {
        // Clone references to channels and registers for use in thread
        let pulse1 = Arc::clone(&self.pulse1);
        let pulse2 = Arc::clone(&self.pulse2);
        let triangle = Arc::clone(&self.triangle);
        let noise = Arc::clone(&self.noise);

        let pulse1_duty = Arc::clone(&self.pulse1_duty);
        let pulse2_duty = Arc::clone(&self.pulse2_duty);
        let noise_mode = Arc::clone(&self.noise_mode);

        // Create a new thread for audio output
        let handle = thread::spawn(move || {
            // Initialize the audio output stream
            let (_stream, stream_handle) = match OutputStream::try_default() {
                Ok(result) => result,
                Err(_) => return, // Exit if can't create audio stream
            };

            // Create a sink for mixing audio sources
            let sink = match Sink::try_new(&stream_handle) {
                Ok(sink) => sink,
                Err(_) => return, // Exit if can't create sink
            };

            // Create sound sources for each channel
            let source1 = PulseWaveSource::new(Arc::clone(&pulse1), Arc::clone(&pulse1_duty))
                .convert_samples::<f32>();

            let source2 = PulseWaveSource::new(Arc::clone(&pulse2), Arc::clone(&pulse2_duty))
                .convert_samples::<f32>();

            let source3 = TriangleWaveSource::new(Arc::clone(&triangle)).convert_samples::<f32>();

            let source4 = NoiseSource::new(Arc::clone(&noise), Arc::clone(&noise_mode))
                .convert_samples::<f32>();

            // Mix the sources together
            let mixed_pulse = source1.mix(source2); // Mix pulse 1 and pulse 2
            let mixed_pulse_triangle = mixed_pulse.mix(source3); // Add triangle
            let mixed_all = mixed_pulse_triangle.mix(source4); // Add noise

            // Apply overall volume (reduce to prevent clipping)
            let final_source = mixed_all.amplify(0.25);

            // Add the mixed source to the sink and start playback
            sink.append(final_source);
            sink.play();

            // Keep the thread alive
            loop {
                thread::sleep(Duration::from_millis(100));
            }
        });

        // Store the thread handle
        *self.audio_thread.lock().unwrap() = Some(handle);
    }

    /// Handles CPU writes to APU registers
    /// This is how the emulated CPU controls sound
    pub fn cpu_write(&mut self, address: u16, data: u8) {
        match address {
            // Pulse 1 channel registers ($4000-$4003)
            0x4000 => {
                // Duty cycle and envelope register
                *self.pulse1_duty.lock().unwrap() = data;

                // Extract volume from lower 4 bits
                let volume = (data & 0x0F) as f32 / 15.0;
                self.pulse1.set_volume(volume);

                // Extract length counter halt bit
                self.pulse1.set_length_counter_halt((data & 0x20) != 0);

                // Enable based on status register
                let status = *self.status.lock().unwrap();
                self.pulse1.set_enabled((status & 0x01) != 0);
            }
            0x4001 => *self.pulse1_sweep.lock().unwrap() = data, // Sweep register
            0x4002 => {
                // Timer low byte
                *self.pulse1_timer_low.lock().unwrap() = data;
                self.update_pulse1_frequency(); // Recalculate frequency
            }
            0x4003 => {
                // Timer high byte and length counter load
                *self.pulse1_timer_high.lock().unwrap() = data & 0b0000_0111; // Only lower 3 bits used
                self.update_pulse1_frequency(); // Recalculate frequency

                // Load length counter from lookup table
                let length_index = (data >> 3) & 0x1F;
                let length_value = self.length_counter_table[length_index as usize];
                self.pulse1.set_length_counter(length_value);

                // Enable channel if status bit is set
                let status = *self.status.lock().unwrap();
                if (status & 0x01) != 0 {
                    self.pulse1.set_enabled(true);
                }
            }

            // Pulse 2 channel registers ($4004-$4007)
            0x4004 => {
                // Duty cycle and envelope register
                *self.pulse2_duty.lock().unwrap() = data;

                // Extract volume from lower 4 bits
                let volume = (data & 0x0F) as f32 / 15.0;
                self.pulse2.set_volume(volume);

                // Extract length counter halt bit
                self.pulse2.set_length_counter_halt((data & 0x20) != 0);

                // Enable based on status register
                let status = *self.status.lock().unwrap();
                self.pulse2.set_enabled((status & 0x02) != 0);
            }
            0x4005 => *self.pulse2_sweep.lock().unwrap() = data, // Sweep register
            0x4006 => {
                // Timer low byte
                *self.pulse2_timer_low.lock().unwrap() = data;
                self.update_pulse2_frequency(); // Recalculate frequency
            }
            0x4007 => {
                // Timer high byte and length counter load
                *self.pulse2_timer_high.lock().unwrap() = data & 0b0000_0111; // Only lower 3 bits used
                self.update_pulse2_frequency(); // Recalculate frequency

                // Load length counter from lookup table
                let length_index = (data >> 3) & 0x1F;
                let length_value = self.length_counter_table[length_index as usize];
                self.pulse2.set_length_counter(length_value);

                // Enable channel if status bit is set
                let status = *self.status.lock().unwrap();
                if (status & 0x02) != 0 {
                    self.pulse2.set_enabled(true);
                }
            }

            // Triangle channel registers ($4008-$400B)
            0x4008 => {
                // Linear counter register
                *self.triangle_linear.lock().unwrap() = data;

                // Extract length counter halt bit
                self.triangle.set_length_counter_halt((data & 0x80) != 0);

                // Enable based on status register
                let status = *self.status.lock().unwrap();
                self.triangle.set_enabled((status & 0x04) != 0);

                // Triangle has fixed volume
                self.triangle.set_volume(0.8);
            }
            0x4009 => {} // Unused register
            0x400A => {
                // Timer low byte
                *self.triangle_timer_low.lock().unwrap() = data;
                self.update_triangle_frequency(); // Recalculate frequency
            }
            0x400B => {
                // Timer high byte and length counter load
                *self.triangle_timer_high.lock().unwrap() = data & 0b0000_0111; // Only lower 3 bits used
                self.update_triangle_frequency(); // Recalculate frequency

                // Load length counter from lookup table
                let length_index = (data >> 3) & 0x1F;
                let length_value = self.length_counter_table[length_index as usize];
                self.triangle.set_length_counter(length_value);

                // Enable channel if status bit is set
                let status = *self.status.lock().unwrap();
                if (status & 0x04) != 0 {
                    self.triangle.set_enabled(true);
                }
            }

            // Noise channel registers ($400C-$400F)
            0x400C => {
                // Volume and envelope register
                *self.noise_volume.lock().unwrap() = data;

                // Extract volume from
                // Extract volume from lower 4 bits
                let volume = (data & 0x0F) as f32 / 15.0;
                self.noise.set_volume(volume);

                // Extract length counter halt bit
                self.noise.set_length_counter_halt((data & 0x20) != 0);

                // Enable based on status register
                let status = *self.status.lock().unwrap();
                self.noise.set_enabled((status & 0x08) != 0);
            }
            0x400D => {} // Unused register
            0x400E => {
                // Noise period and mode register
                *self.noise_period.lock().unwrap() = data;

                // Extract mode flag (bit 7)
                *self.noise_mode.lock().unwrap() = (data & 0x80) != 0;

                // Update the noise frequency based on period
                self.update_noise_frequency();
            }
            0x400F => {
                // Length counter load register
                *self.noise_length.lock().unwrap() = data;

                // Load length counter from lookup table
                let length_index = (data >> 3) & 0x1F;
                let length_value = self.length_counter_table[length_index as usize];
                self.noise.set_length_counter(length_value);

                // Enable channel if status bit is set
                let status = *self.status.lock().unwrap();
                if (status & 0x08) != 0 {
                    self.noise.set_enabled(true);
                }
            }

            // APU status register ($4015)
            0x4015 => {
                // Status register controls which channels are enabled
                *self.status.lock().unwrap() = data & 0x0F; // Only lower 4 bits are used

                // Enable or disable pulse 1 channel
                if (data & 0x01) == 0 {
                    self.pulse1.set_length_counter(0); // Reset length counter if disabled
                }
                self.pulse1.set_enabled((data & 0x01) != 0);

                // Enable or disable pulse 2 channel
                if (data & 0x02) == 0 {
                    self.pulse2.set_length_counter(0); // Reset length counter if disabled
                }
                self.pulse2.set_enabled((data & 0x02) != 0);

                // Enable or disable triangle channel
                if (data & 0x04) == 0 {
                    self.triangle.set_length_counter(0); // Reset length counter if disabled
                }
                self.triangle.set_enabled((data & 0x04) != 0);

                // Enable or disable noise channel
                if (data & 0x08) == 0 {
                    self.noise.set_length_counter(0); // Reset length counter if disabled
                }
                self.noise.set_enabled((data & 0x08) != 0);
            }

            // Frame counter register ($4017)
            0x4017 => {
                *self.frame_counter.lock().unwrap() = data;
                // Note: The frame counter control isn't fully implemented here
            }

            // Ignore any other addresses
            _ => {}
        }
    }

    /// Handles CPU reads from APU registers
    /// This allows the emulated CPU to get status information
    pub fn cpu_read(&self, address: u16) -> u8 {
        match address {
            // Return the current register values for all registers
            0x4000 => *self.pulse1_duty.lock().unwrap(),
            0x4001 => *self.pulse1_sweep.lock().unwrap(),
            0x4002 => *self.pulse1_timer_low.lock().unwrap(),
            0x4003 => *self.pulse1_timer_high.lock().unwrap(),
            0x4004 => *self.pulse2_duty.lock().unwrap(),
            0x4005 => *self.pulse2_sweep.lock().unwrap(),
            0x4006 => *self.pulse2_timer_low.lock().unwrap(),
            0x4007 => *self.pulse2_timer_high.lock().unwrap(),
            0x4008 => *self.triangle_linear.lock().unwrap(),
            0x4009 => 0, // Unused register
            0x400A => *self.triangle_timer_low.lock().unwrap(),
            0x400B => *self.triangle_timer_high.lock().unwrap(),
            0x400C => *self.noise_volume.lock().unwrap(),
            0x400D => 0, // Unused register
            0x400E => *self.noise_period.lock().unwrap(),
            0x400F => *self.noise_length.lock().unwrap(),

            // Status register ($4015) - returns active channel flags
            0x4015 => {
                let mut status = *self.status.lock().unwrap();

                // Set bit 0 if pulse 1 length counter is active
                if *self.pulse1.length_counter.lock().unwrap() > 0 {
                    status |= 0x01;
                }

                // Set bit 1 if pulse 2 length counter is active
                if *self.pulse2.length_counter.lock().unwrap() > 0 {
                    status |= 0x02;
                }

                // Set bit 2 if triangle length counter is active
                if *self.triangle.length_counter.lock().unwrap() > 0 {
                    status |= 0x04;
                }

                // Set bit 3 if noise length counter is active
                if *self.noise.length_counter.lock().unwrap() > 0 {
                    status |= 0x08;
                }

                status
            }

            // Frame counter register ($4017)
            0x4017 => *self.frame_counter.lock().unwrap(),

            // Return 0 for other addresses
            _ => 0,
        }
    }

    /// Converts timer values to actual frequency in Hz
    /// The NES hardware uses timer values that are inversely related to frequency
    fn get_frequency_from_timer(high: u8, low: u8) -> f32 {
        // Combine high and low bytes into a 16-bit timer value
        let timer = ((high as u16) << 8) | low as u16;

        if timer == 0 {
            0.0 // Avoid division by zero
        } else {
            // NES CPU clock rate (1.789773 MHz) divided by 16 times the timer value plus 1
            // This gives the actual frequency in Hz
            1_789_773.0 / (16.0 * (timer as f32 + 1.0))
        }
    }

    /// Updates pulse 1 channel frequency based on timer registers
    fn update_pulse1_frequency(&self) {
        let high = *self.pulse1_timer_high.lock().unwrap();
        let low = *self.pulse1_timer_low.lock().unwrap();
        let freq = Self::get_frequency_from_timer(high, low);
        self.pulse1.set_frequency(freq);
    }

    /// Updates pulse 2 channel frequency based on timer registers
    fn update_pulse2_frequency(&self) {
        let high = *self.pulse2_timer_high.lock().unwrap();
        let low = *self.pulse2_timer_low.lock().unwrap();
        let freq = Self::get_frequency_from_timer(high, low);
        self.pulse2.set_frequency(freq);
    }

    /// Updates triangle channel frequency based on timer registers
    /// Triangle uses a slightly different formula than pulse channels
    fn update_triangle_frequency(&self) {
        let high = *self.triangle_timer_high.lock().unwrap();
        let low = *self.triangle_timer_low.lock().unwrap();
        let timer = ((high as u16) << 8) | low as u16;

        let freq = if timer == 0 {
            0.0 // Avoid division by zero
        } else {
            // Triangle channel divides by 32 instead of 16
            1_789_773.0 / (32.0 * (timer as f32 + 1.0))
        };

        self.triangle.set_frequency(freq);
    }

    /// Updates noise channel frequency based on period register
    fn update_noise_frequency(&self) {
        // Get the period index from lower 4 bits of noise period register
        let period_idx = *self.noise_period.lock().unwrap() & 0x0F;

        // Lookup table for noise periods based on the NES hardware
        let period_table: [u16; 16] = [
            4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
        ];

        // Get the actual period from the table
        let period = period_table[period_idx as usize];

        // Convert period to frequency
        let freq = 1_789_773.0 / (period as f32 * 2.0);

        self.noise.set_frequency(freq);
    }

}

/// Cleanup when APU is dropped
/// Ensures all sounds are stopped properly
impl Drop for Apu {
    fn drop(&mut self) {
        // Disable all channels to stop any sounds
        self.pulse1.set_enabled(false);
        self.pulse2.set_enabled(false);
        self.triangle.set_enabled(false);
        self.noise.set_enabled(false);
    }
}
