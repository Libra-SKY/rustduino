//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Nikhil Gupta, Indian Institute of Technology Kanpur
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Affero General Public License as published
//     by the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Affero General Public License for more details.
//
//     You should have received a copy of the GNU Affero General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>
use bit_field::BitField;
use core;
use core::ptr::{read_volatile};
use volatile::Volatile;
use rustduino::atmega2560p::{usart_initialize,usart_initialize::Usart};

impl Usart{
   ///This function enables the reciever function of microcontroller, whithout enabling it no communication is possible.
   pub fn recieve_enable(&mut self){
    unsafe {
        self.stctrlh.update(|ctrl| {
            ctrl.set_bit(4, true);
        });
    }
   }
   ///This function is used to recieve data of one frame. 
   ///Either 5 to 8 bits and 9 bits of data can be recieved from this function.
   ///In case of 5 to 8 bits this function returns u8.
   ///In case of 9 bits it retuns u32 of which first 9 bits are data recieved and remaining bits are insignificant.
   ///In case ,if an frame error or parity error occurs, this function returns -1.
   pub fn recieve_data(&mut self)->Option<Volatile<u8>,Volatile<u32>>{
    unsafe{
        let  ucsrc=read_volatile(&self.ucsrc);
        let  ucsrb=read_volatile(&self.ucsrb);
        if ucsrc.gets_bits(1..3)==0b11 && ucsrb.get_bit(2){
            let ucsra=read_volatile(&self.ucsra);

            while !(ucsra.getbit(7)){};
            let ucsra=read_volatile(&self.ucsra);
            let ucsrb=read_volatile(&self.ucsrb);
            let udr=read_volatile(&mut self.udr);
            if ucsra.get_bits(2..5)!=0b000{
                Some(-1)
            }
            else{
                let rxb8=ucsrb.get_bits(1..2);
                udr=udr.set_bits(8..9,rxb8);
                Some(udr);
            }
        }
        else{
            let ucsra=read_volatile(&self.ucsra);
            while !(ucsra.getbit(7)){};
           if error_check(&mut self){
               Some(-1)
           }
           if parity_check(&mut self){
              Some(-1)
           }
            Some(read_volatile(&mut self.udr))
            
        }
    }
} 
 ///This function can be used to check frame error(excluding parity checker).
 ///It returns true if error occurs,else false.
  pub fn error_check(&mut self)->bool{
      unsafe{
      let ucsra=read_volatile(&self.ucsra);
      if ucsra.getbits(2..4)!=0b000{
          true
      }
      else{
          false
      }
    }
  }
  ///This function can be used to check parity error.
  ///It returns true if error occurs,else false.
  pub fn parity_check(&mut self)->bool{
      unsafe{
    let ucsra=read_volatile(&self.ucsra);
    if ucsra.getbit(4)==0b1{
        true
    } 
    else{
        false
    }
   }
  }
 ///This function disables the reciever function of microcontroller.
  pub fn recieve_disable(&mut self){
    unsafe {
        self.stctrlh.update(|ctrl| {
            ctrl.set_bit(4, false);
        });
    }
   }
}