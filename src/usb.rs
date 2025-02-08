use std::time::Duration;

// finds the only out interface on Winbond keyboards
fn interface_interrupt(vid: u16, pid: u16, buffer: &[u8; 64]) -> bool {
    for device in rusb::devices().unwrap().iter() {
        if device.device_descriptor().unwrap().vendor_id() != vid || device.device_descriptor().unwrap().product_id() != pid {
            continue;
        }

        match device.active_config_descriptor() {
            Ok(config_descriptor) => {
                for c in config_descriptor.interfaces() {
                    for d in c.descriptors() {
                        for e in d.endpoint_descriptors() {
                            if e.direction() == rusb::Direction::In {
                                continue;
                            }

                            let handle = rusb::open_device_with_vid_pid(vid, pid).unwrap();

                            if handle.kernel_driver_active(c.number()).unwrap() {
                                let _ = handle.detach_kernel_driver(c.number());
                            }
                            
                            let return_value;
                            match handle.write_interrupt(e.number(), buffer, Duration::new(1, 0)) {
                                Ok(bytes_written) => {
                                    println!("sent {} bytes to keyboard OUT endpoint", bytes_written);
                                    return_value = true;
                                }
                                Err(err) => {
                                    println!("error writing bytes to keyboard OUT endpoint: {}", err);
                                    return_value = false;
                                }
                            }

                            let _ = handle.attach_kernel_driver(c.number());
                            return return_value;
                        }
                    }
                }
            }
            Err(err) => { println!("error getting device config descriptor: {}", err); }
        }
    }
    return false;
}


fn detect_keyboard(vid: &mut Option<u16>, pid: &mut Option<u16>) {
    for device in rusb::devices().unwrap().iter() {

        let device_descriptor = device.device_descriptor();

        if let Ok(dd) = device_descriptor {
            if dd.vendor_id() != 0x0416 {
                continue; // must be winbond vendor
            }

            let handle = rusb::open_device_with_vid_pid(dd.vendor_id(), dd.product_id());
            if handle.as_ref().is_none() {
                continue;
            }

            if let Some(handle_ref) = handle.as_ref() {
                let product_string = handle_ref.read_product_string_ascii(&dd);
                match product_string {
                    Ok(str) => {
                        if str == "Gaming Keyboard" {
                            *pid = Some(dd.product_id());
                            *vid = Some(dd.vendor_id());
                            break;
                        }
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }

        }

    }
}


// returns true if success, else false
pub fn send_keyboard_values(
    mode: u8,
    brightness: u8,
    speed: u8,
    colour_1_r: u8,
    colour_1_g: u8,
    colour_1_b: u8,
    colour_2_r: u8,
    colour_2_g: u8,
    colour_2_b: u8,
    direction: u8,
    rainbow: bool
) -> bool {
    // auto-detected values
    let mut vid: Option<u16> = None;
    let mut pid: Option<u16> = None;

    detect_keyboard(&mut vid, &mut pid);

    if vid.is_none() {
        println!("error during device detection - vendor ID is none");
        return false;
    } else {
        println!("successfully detected the device - {:#04}:{:#04}", vid.unwrap(), pid.unwrap());
    }

    let buffer: [u8; 64] = [0x1,  0x7,  0x0, 0x0, 0x0, 0xe, mode, brightness, speed, colour_1_r, colour_1_g, colour_1_b, colour_2_r, colour_2_g, colour_2_b, direction, if rainbow { 0x01 } else  { 0x00 }, 0x0, 0x0, 0x0,  0x0, 0x0, 0x0,  0x0,  0x0,  0x0, 0x0,  0x0,  0x0, 0x0, 0x0, 0x0, 0x0,  0x0, 0x0, 0x0,  0x0,  0x0,  0x0, 0x0,  0x0,  0x0, 0x0, 0x0, 0x0, 0x0,  0x0, 0x0, 0x0,  0x0,  0x0,  0x0, 0x0,  0x0,  0x0, 0x0, 0x0, 0x0, 0x0,  0x0, 0x0, 0x0,  0x0,  0x0];
    return interface_interrupt(vid.unwrap(), pid.unwrap(), &buffer);
    // returns success of sending bytes to device
}
