use std::i64;
use std::i16;
use std::thread::sleep;
use std::time::Duration;
#[allow(non_snake_case)]
#[warn(unused_unsafe)]
#[link(name="bass", kind = "static")]
extern "C"
{
    fn BASS_Init(arg: i16, arg: i32, arg: i16, arg: i16, arg: i16) -> bool;
    fn BASS_Start() -> bool;
    fn BASS_StreamCreateFile(arg: i16, arg: String, arg: i16, arg: i16, arg: i16) -> i32;
    fn BASS_ChannelPlay(arg: i32, arg: bool );
}
fn main() 
{
        
        loop {
            unsafe { BASS_Init(-1, 44100, 0, 0, 0) };
            unsafe { BASS_Start() };
            let unicodestring = String::from("SCP5000Music.mp3");
            let xlk: i32 = unsafe { BASS_StreamCreateFile(0, unicodestring, 0, 0, 4) };
            unsafe { BASS_ChannelPlay(xlk, false) };
            let floatx: f64 = 3000_f64;
            let dur: Duration = Duration::from_secs_f64(floatx);
            sleep(dur);
        }
}
