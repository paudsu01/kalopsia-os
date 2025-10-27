use crate::utils::Port;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref PICS: Mutex<Pics> = Mutex::new({
        let mut pics = Pics::new();
        pics.remap();
        pics
    });
}

const MASTER_COMMAND_PORT: u16 = 0x20;
const MASTER_DATA_PORT: u16 = MASTER_COMMAND_PORT + 1;
const SLAVE_COMMAND_PORT: u16 = 0xA0;
const SLAVE_DATA_PORT: u16 = SLAVE_COMMAND_PORT + 1;

const PIC_EOI: u8 = 0x20; // End of Interrupt command code
/// Interrupt Vector Number conflict with IDT handlers for exceptions
/// Therefore, the PIC needs to be configured to an offset (done with `remap` method)
/// For example, timer uses line 0 of the primary PIC. This means that it arrives at the CPU as interrupt 32 (0+ 32) with offset of 32
const MASTER_OFFSET: u8 = 0x20;
const SLAVE_OFFSET: u8 = MASTER_OFFSET + 8;

#[repr(u8)]
#[allow(dead_code)]
pub enum Interrupts {
    Timer = MASTER_OFFSET,
    Keyboard,
}
/// Struct to represent the 8259 PIC
/// The 8259 PIC contains two IC (master and slave)
/// The master use command port: 0x20 and data port: 0x21
/// The slave uses command port: 0x20 and data port: 0x21
struct Pic {
    command: Port,
    data: Port,
}

enum Mode {
    Command,
    Data,
}

#[allow(dead_code)]
impl Pic {
    fn new(command: u16, data: u16) -> Self {
        Pic {
            command: Port::new(command),
            data: Port::new(data),
        }
    }

    /// Send EOI signal to the PIC so that the PIC knows the interrupt was processed
    /// and so that the PIC knows we are ready to receive the next interrupt
    fn end_of_interrupt(&mut self) {
        self.command.writeb(PIC_EOI);
    }

    /// Write `byte` to either the command port or the data port based on the mode provided
    fn writeb(&mut self, mode: Mode, byte: u8) {
        match mode {
            Mode::Data => self.data.writeb(byte),
            Mode::Command => self.command.writeb(byte),
        };
    }

    /// Read a byte from either the command port or the data port based on the mode provided
    fn readb(&mut self, mode: Mode) -> u8 {
        match mode {
            Mode::Data => self.data.readb(),
            Mode::Command => self.command.readb(),
        }
    }

    /// Mask every interrupt
    fn disable(&mut self) {
        // mask every interrupt (there are 8 lines)
        self.data.writeb(0xff);
    }

    /// Unmask every interrupt
    fn unmask(&mut self) {
        self.data.writeb(0);
    }
}

struct Pics {
    master: Pic,
    slave: Pic,
    master_offset: u8,
    slave_offset: u8,
}

#[allow(dead_code)]
impl Pics {
    fn new() -> Self {
        Pics {
            master: Pic::new(MASTER_COMMAND_PORT, MASTER_DATA_PORT),
            slave: Pic::new(SLAVE_COMMAND_PORT, SLAVE_DATA_PORT),
            master_offset: MASTER_OFFSET,
            slave_offset: SLAVE_OFFSET,
        }
    }

    /// Remap the PIC to have the correct specified offsets
    // Following the initialisation code from: https://wiki.osdev.org/8259_PIC#Code_Examples
    fn remap(&mut self) {
        let _init_command: u8 = 0x11;
        let _mode_8086: u8 = 0x01;
        // Need to add delay between PIC data port writes
        // Working around by writing garbage data to port 0x80 (source: http://www.faqs.org/docs/Linux-mini/IO-Port-Programming.html#Accessing-The-Ports)
        let mut garbage_port: Port = Port::new(0x80);
        let mut io_wait = || {
            garbage_port.writeb(00);
        };

        // give the two PICs the initialise command (code 0x11)
        self.master.writeb(Mode::Command, _init_command);
        io_wait();
        self.slave.writeb(Mode::Command, _init_command);
        io_wait();

        /* Now, the PICs wait for 3 bytes on the data port(ICW2, ICW3, ICW4) */

        // ICW2: vector offset
        self.master.writeb(Mode::Data, self.master_offset);
        io_wait();
        self.slave.writeb(Mode::Data, self.slave_offset);
        io_wait();

        // ICW3: tell how it is wired to master/slaves
        // Slave is at IRQ2(third line -> 0000 0100)
        self.master.writeb(Mode::Data, 0b100);
        io_wait();
        // Tell slave its on master's third input line(input #2)
        self.slave.writeb(Mode::Data, 2);
        io_wait();

        // ICW4: tell PICs to use 8086 mode
        self.master.writeb(Mode::Data, _mode_8086);
        io_wait();
        self.slave.writeb(Mode::Data, _mode_8086);
        io_wait();

        // Unmask both PICs
        self.master.unmask();
        self.slave.unmask();
    }

    /// EOI signal should be sent so that the PIC knows we are ready to receive the next interrupt
    /// Notify master PIC only if the IRQ came from the Master PIC
    /// Otherwise, notify both master and slave PIC
    fn end_of_interrupt(&mut self, irq: u8) {
        if irq >= self.slave_offset {
            self.slave.end_of_interrupt();
        }
        self.master.end_of_interrupt();
    }
}
