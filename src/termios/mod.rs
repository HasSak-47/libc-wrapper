use bitflags::bitflags;
use libc::{cc_t, speed_t, tcflag_t, tcgetattr, tcsetattr, termios};
use std::{
    mem::zeroed,
    os::fd::{self, AsRawFd},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to set term attributes")]
    TermiosSet,

    #[error("failed to get term attributes")]
    TermiosGet,
}
pub type Result<T> = std::result::Result<T, Error>;

bitflags! {
    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
    pub struct Actions : i32 {
        const TCSAFLUSH = 2;
    }

    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
    pub struct InputMode: u32 {
        const IgnoreBreak = libc::IGNBRK;
        const BreakInt = libc::BRKINT;
        const IgnoreParity = libc::IGNPAR;
        const MarkParity = libc::PARMRK;
        const InputParityCheck = libc::INPCK;
        const StripEightBit = libc::ISTRIP;
        const NewLinetoCarriageReturn = libc::INLCR;
        const IgnoreCarriageReturn = libc::IGNCR;
        const CarriageReturnToNewLine = libc::ICRNL;
        const EnableXONOut = libc::IXON;
        const RestartOutput= libc::IXANY;
        const EnableXONIn = libc::IXOFF;
        const InputIsU8= libc::IUTF8;
        const RinBell= libc::IMAXBEL;

        // const ForceLowerCase = libc::IUCLC;
    }

    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
    pub struct OutputMode: u32 {
        const UserDefProcess= libc::OPOST;
        const ForceLowercase = libc::OLCUC;
        const NewLineCarriageReturn = libc::ONLCR;
        const CarriageReturnNewLine = libc::OCRNL;
        const NoCarriageReturn = libc::ONOCR;
        const NewLineReturn = libc::ONLRET;
        const FillDelay = libc::OFILL;
        const FillisDel= libc::OFDEL;
        const NewLineDelayMask = libc::NLDLY;
        const CarriageReturnDelayMask = libc::CRDLY;
        const TabDelayMask = libc::TABDLY;
        const BackspaceDelayMask = libc::BSDLY;
        const VerticalTabDelayMask = libc::VTDLY;
        const FormFeedDelayMask = libc::FFDLY;
    }

    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
    pub struct ControlMode: u32 {
        const BaudSpeedMask = libc::CBAUD;
        const ExtraBaudSpeedMask = libc::CBAUDEX;
        const CharacterSizeMask = libc::CSIZE;
        const TwoStopBits = libc::CSTOPB;
        const EnableReceiver = libc::CREAD;
        const EnableParityBit = libc::PARENB;
        const OddParity = libc::PARODD;
        const HangUpAfterClose = libc::HUPCL;
        const IgnoreModemControlLines = libc::CLOCAL;
        const MaskInputMask = libc::CIBAUD;
        const UseStickParity = libc::CMSPAR;
        const EnaleHardwareControl = libc::CRTSCTS;

        // const LOBLK = libc::LOBLK;
    }

    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
    pub struct LocalMode: tcflag_t{
        const GenerateSignals= libc::ISIG;
        const EnableCanonMode = libc::ICANON;
        const EchoInput = libc::ECHO;
        const ErasePreciding = libc::ECHOE;
        const KillEraces = libc::ECHOK;
        const EchoNewLine = libc::ECHONL;
        const EchoAsX = libc::ECHOCTL;
        const PrintAsDeleted = libc::ECHOPRT;
        const KillIsEcho = libc::ECHOKE;
        const OutputIsBeingFlushed = libc::FLUSHO;
        const NoFlush = libc::NOFLSH;
        const SendSIGTTOU= libc::TOSTOP;
        const ReprintQueue = libc::PENDIN;
        const UserDefProcess = libc::IEXTEN;

        // const XCASE = libc::XCASE;
        // const DEFECHO = libc::DEFECHO;
    }
}

#[derive(Default, Debug, Clone)]
pub struct Termios {
    pub input: InputMode,
    pub output: OutputMode,
    pub control: ControlMode,
    pub local: LocalMode,
    pub out_speed: speed_t,
    pub in_speed: speed_t,
    pub line_discipline: cc_t,
    pub control_characters: [cc_t; libc::NCCS],
}

impl Termios {
    pub fn set_attr<F>(&self, fd: F, actions: Actions) -> self::Result<()>
    where
        F: fd::AsFd,
    {
        let t = termios {
            c_iflag: self.input.bits(),
            c_oflag: self.output.bits(),
            c_cflag: self.control.bits(),
            c_lflag: self.local.bits(),
            c_cc: self.control_characters,
            c_line: self.line_discipline,
            c_ispeed: self.in_speed,
            c_ospeed: self.out_speed,
        };

        let fd = fd.as_fd().as_raw_fd();
        unsafe {
            if tcsetattr(fd, actions.bits(), &t) == -1 {
                return Err(Error::TermiosSet);
            }
        }

        Ok(())
    }

    pub fn get_attr<F>(fd: F) -> self::Result<Self>
    where
        F: fd::AsFd,
    {
        unsafe {
            let fd = fd.as_fd().as_raw_fd();

            let mut t: termios = zeroed();
            if tcgetattr(fd, &mut t) == -1 {
                return Err(Error::TermiosGet);
            }

            return Ok(Termios {
                input: InputMode::from_bits_truncate(t.c_iflag),
                output: OutputMode::from_bits_truncate(t.c_oflag),
                control: ControlMode::from_bits_truncate(t.c_cflag),
                local: LocalMode::from_bits_truncate(t.c_lflag),
                control_characters: t.c_cc,
                line_discipline: t.c_line,
                in_speed: t.c_ispeed,
                out_speed: t.c_ospeed,
            });
        }
    }
}
