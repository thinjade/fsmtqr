#![windows_subsystem = "windows"]

//use barcoders::generators::image::*;
use barcoders;
use barcoders::sym::code128::*;
use encoding::all::GB18030;
use encoding::{EncoderTrap, Encoding};
use fltk::{app, button, frame, image, input, prelude::*, window};
use qrcode_generator::QrCodeEcc;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;
//use urlencoding;

fn main() {
    let img_path = "FM_IMG";

    let version = env!("CARGO_PKG_VERSION");
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(600, 510)
        .center_screen(); //new(200, 200, 600, 510, "");
    wind.set_label(format!("FS&MT二维码生成工具 {}", version).as_str());
    let w_x = wind.x();
    let w_y = wind.y();
    let (s, r) = app::channel::<i32>();
    let mut mb = fltk::menu::SysMenuBar::default().with_size(800, 35);
    mb.set_frame(fltk::enums::FrameType::FlatBox);
    mb.set_text_size(12);
    mb.add(
        "关于",
        fltk::enums::Shortcut::None,
        fltk::menu::MenuFlag::Normal,
        move |_| {
            /***
            fltk::dialog::message_title("关于");
            fltk::dialog::message(w_x + 50, w_y + 50,format!(
                    "FS&MT二维码生成工具 {}\n\n二维码保存在{}文件夹\n\nCopyright@@ThinJade",
                    version, img_path
                ).as_str());
                ***/
            let mut help_dlg =
                fltk::window::Window::new(w_x + 50, w_y + 50, 300, 150, "关于").center_screen();
            let mut help_f = frame::Frame::default().with_size(300, 75);
            help_f.set_label(
                format!(
                    "\nFS&MT二维码生成工具 {}\n\n二维码保存在{}文件夹\n\nCopyright@@ThinJade",
                    version, img_path
                )
                .as_str(),
            );
            let mut ok_but = button::Button::new(120, 105, 50, 28, "确定");
            help_dlg.end();
            help_dlg.make_modal(true);
            help_dlg.show();
            ok_but.set_callback(move |_| {
                help_dlg.hide();
            });

            /***
                let mut help = fltk::dialog::HelpDialog::new(w_x + 50, w_y + 50, 300, 150);
                let h_t=format!(
                        "FS&MT二维码生成工具 {}\n\n二维码保存在{}文件夹\n\nCopyright@@ThinJade",
                        version, img_path);
            help.set_value(&h_t); // this takes html
            help.show();
            while help.shown() {
                app::wait();
            }***/
        },
    );

    let mut choice = fltk::menu::Choice::new(95, 30, 85, 30, "设备类型：");
    choice.add_emit(
        "条码",
        fltk::enums::Shortcut::None,
        fltk::menu::MenuFlag::Normal,
        s,
        0,
    );
    choice.add_emit(
        "量贩主柜",
        fltk::enums::Shortcut::None,
        fltk::menu::MenuFlag::Normal,
        s,
        1,
    );
    choice.add_emit(
        "量贩副柜",
        fltk::enums::Shortcut::None,
        fltk::menu::MenuFlag::Normal,
        s,
        2,
    );
    choice.add_emit(
        "3门冷柜",
        fltk::enums::Shortcut::None,
        fltk::menu::MenuFlag::Normal,
        s,
        3,
    );
    choice.add_emit(
        "4门冷柜",
        fltk::enums::Shortcut::None,
        fltk::menu::MenuFlag::Normal,
        s,
        4,
    );
    choice.add_emit(
        "美团柜",
        fltk::enums::Shortcut::None,
        fltk::menu::MenuFlag::Normal,
        s,
        5,
    );
    choice.add_emit(
        "二维码",
        fltk::enums::Shortcut::None,
        fltk::menu::MenuFlag::Normal,
        s,
        6,
    );
    choice.set_value(0);

    let mut input_code = input::Input::new(225, 30, 210, 30, "");
    input_code.set_label_size(12);

    let mut but = button::Button::new(470, 30, 80, 30, "生成条码");
    but.set_label_size(12);
    let mut iframe = frame::Frame::new(10, 60, 580, 450, "");

    wind.end();
    wind.show();

    but.emit(s, 999);

    let s1 = s.clone();

    input_code.set_trigger(fltk::enums::CallbackTrigger::EnterKey);
    input_code.emit(s, 999);

    input_code.handle(move |_, ev| match ev {
        fltk::enums::Event::Push => {
            if app::event_mouse_button() == app::MouseButton::Right {
                s1.send(888);
            }
            true
        }
        _ => true,
    });

    //app.run().unwrap();
    while app.wait() {
        if let Some(r) = r.recv() {
            //println!("{:?}", r);
            match r {
                0 => {
                    but.set_label("生成条码");
                    input_code.set_label("");
                    app.redraw();
                }
                1 => {
                    but.set_label("生成二维码");
                    input_code.set_label("YST");
                    app.redraw();
                }
                2 => {
                    but.set_label("生成二维码");
                    input_code.set_label("YST");
                    app.redraw();
                }
                3 => {
                    but.set_label("生成二维码");
                    input_code.set_label("NFDM");
                    app.redraw();
                }
                4 => {
                    but.set_label("生成二维码");
                    input_code.set_label("NFDM");
                    app.redraw();
                }
                5 => {
                    but.set_label("生成二维码");
                    input_code.set_label("");
                    app.redraw();
                }
                6 => {
                    but.set_label("生成二维码");
                    input_code.set_label("");
                    app.redraw();
                }
                888 => {
                    fltk::app::paste_text(&input_code);
                }
                999 => {
                    match fltk::app::focus() {
                        Some(_wig) => { /***println!(
                                 "{:?}\n{:?}-{:?}",
                                 fltk::app::event_coords(),
                                 fltk::app::event(),
                                 wig.x()
                             )***/
                        }
                        None => {
                            //println!("unknown")
                        }
                    }
                    match fs::create_dir(img_path) {
                        Err(_why) => {}
                        Ok(_) => {}
                    }

                    let mut code = input_code.value().to_string();

                    code = code.trim().to_string();
                    let len = code.len();
                    if len == 0 {
                        //println!("输入为空");
                    } else {
                        if choice.value() == 0 {
                            match Code128::new(format!("Ɓ{}", code)) {
                                Ok(barcode) => {
                                    //let png = barcoders::generators::image::Image::png(80); // You must specify the height in pixels.
                                    let png = barcoders::generators::image::Image::PNG {
                                        height: 400,
                                        xdim: 3,
                                        rotation: barcoders::generators::image::Rotation::Zero,
                                        foreground: barcoders::generators::image::Color::new([
                                            0, 0, 0, 255,
                                        ]),
                                        background: barcoders::generators::image::Color::new([
                                            255, 255, 255, 255,
                                        ]),
                                    };
                                    let encoded = barcode.encode();

                                    let bytes = png.generate(&encoded[..]).unwrap();
                                    //let ncode = urlencoding::encode(&code);
                                    let ncode = code
                                        .replace("%", "%25")
                                        .replace("<", "%3C")
                                        .replace(">", "%3E")
                                        .replace(":", "%3A")
                                        .replace("\"", "%22")
                                        .replace("/", "%2F")
                                        .replace("\\", "%5C")
                                        .replace("|", "%7C")
                                        .replace("?", "%3F")
                                        .replace("*", "%2A");

                                    let fo = format!("{}/CODE128-{}.png", img_path, ncode);
                                    let file = File::create(&Path::new(&fo)).unwrap();
                                    let mut writer = BufWriter::new(file);
                                    writer.write(&bytes[..]).unwrap();
                                    writer.flush().unwrap();

                                    match image::SharedImage::load(format!(
                                        "{}/CODE128-{}.png",
                                        img_path, ncode
                                    )) {
                                        Ok(mut nimage) => {
                                            nimage.scale(550, 400, true, true);
                                            iframe.set_label_color(fltk::enums::Color::Red);

                                            iframe.set_image(Some(nimage));
                                            iframe.set_label(
                                                format!(
                                                    "{}\n条码大图已保存到{}文件夹",
                                                    code, img_path
                                                )
                                                .as_str(),
                                            );
                                        }
                                        Err(_e) => {
                                            //println!("{:?}", e);
                                        }
                                    }
                                }
                                Err(_) => {
                                    //println!("err 2");
                                }
                            }
                        } else if choice.value() == 1 {
                            let fsdata = format!(
                                "{}{}{}",
                                "设备型号：XBY-ZJ-NN100FS-3\r\n资产编码：YST",
                                code,
                                "\r\n资产属性：养生堂集团所有\r\n服务电话：95077"
                            );
                            //let ncode = urlencoding::encode(&code);
                            let ncode = code
                                .replace("%", "%25")
                                .replace("<", "%3C")
                                .replace(">", "%3E")
                                .replace(":", "%3A")
                                .replace("\"", "%22")
                                .replace("/", "%2F")
                                .replace("\\", "%5C")
                                .replace("|", "%7C")
                                .replace("?", "%3F")
                                .replace("*", "%2A");

                            qrcode_generator::to_png_to_file(
                                GB18030.encode(&fsdata, EncoderTrap::Strict).unwrap(),
                                QrCodeEcc::Low,
                                1024,
                                format!("{}{}{}{}", img_path, "/FS-ZJ-", ncode, ".png"),
                            )
                            .unwrap();
                            match image::SharedImage::load(format!(
                                "{}/FS-ZJ-{}.png",
                                img_path, ncode
                            )) {
                                Ok(mut nimage) => {
                                    nimage.scale(550, 400, true, true);
                                    iframe.set_label_color(fltk::enums::Color::Red);

                                    iframe.set_image(Some(nimage));
                                    iframe.set_label(
                                        format!(
                                            "FS-ZJ-{}\n二维码大图已保存到{}文件夹",
                                            code, img_path
                                        )
                                        .as_str(),
                                    );
                                }
                                Err(_) => {}
                            }
                        } else if choice.value() == 2 {
                            let fsdata = format!(
                                "{}{}{}",
                                "设备型号：XBY-FJ-NN114FS-14\r\n资产编码：YST",
                                code,
                                "\r\n资产属性：养生堂集团所有\r\n服务电话：95077"
                            );
                            //let ncode = urlencoding::encode(&code);
                            let ncode = code
                                .replace("%", "%25")
                                .replace("<", "%3C")
                                .replace(">", "%3E")
                                .replace(":", "%3A")
                                .replace("\"", "%22")
                                .replace("/", "%2F")
                                .replace("\\", "%5C")
                                .replace("|", "%7C")
                                .replace("?", "%3F")
                                .replace("*", "%2A");

                            qrcode_generator::to_png_to_file(
                                GB18030.encode(&fsdata, EncoderTrap::Strict).unwrap(),
                                QrCodeEcc::Low,
                                1024,
                                format!("{}{}{}{}", img_path, "/FS-FJ-", ncode, ".png"),
                            )
                            .unwrap();
                            match image::SharedImage::load(format!(
                                "{}/FS-FJ-{}.png",
                                img_path, ncode
                            )) {
                                Ok(mut nimage) => {
                                    nimage.scale(550, 400, true, true);
                                    iframe.set_label_color(fltk::enums::Color::Red);

                                    iframe.set_image(Some(nimage));
                                    iframe.set_label(
                                        format!(
                                            "FS-FJ-{}\n二维码大图已保存到{}文件夹",
                                            code, img_path
                                        )
                                        .as_str(),
                                    );
                                }
                                Err(_) => {}
                            }
                        } else if choice.value() == 3 {
                            let fsdata = format!(
                                "{}{}{}",
                                "设备型号：Y3-D1500-RB\n资产编号：NFDM",
                                code,
                                "\n资产属性：农夫山泉所有"
                            );
                            //let ncode = urlencoding::encode(&code);
                            let ncode = code
                                .replace("%", "%25")
                                .replace("<", "%3C")
                                .replace(">", "%3E")
                                .replace(":", "%3A")
                                .replace("\"", "%22")
                                .replace("/", "%2F")
                                .replace("\\", "%5C")
                                .replace("|", "%7C")
                                .replace("?", "%3F")
                                .replace("*", "%2A");

                            qrcode_generator::to_png_to_file(
                                &fsdata,
                                QrCodeEcc::Low,
                                1024,
                                format!("{}{}{}{}", img_path, "/FS-3D-", ncode, ".png"),
                            )
                            .unwrap();
                            match image::SharedImage::load(format!(
                                "{}/FS-3D-{}.png",
                                img_path, ncode
                            )) {
                                Ok(mut nimage) => {
                                    nimage.scale(550, 400, true, true);
                                    iframe.set_label_color(fltk::enums::Color::Red);

                                    iframe.set_image(Some(nimage));
                                    iframe.set_label(
                                        format!(
                                            "FS-3D-{}\n二维码大图已保存到{}文件夹",
                                            code, img_path
                                        )
                                        .as_str(),
                                    );
                                }
                                Err(_) => {}
                            }
                        } else if choice.value() == 4 {
                            let fsdata = format!(
                                "{}{}{}",
                                "设备型号：Y4-D2100-RB\n资产编号：NFDM",
                                code,
                                "\n资产属性：农夫山泉所有"
                            );
                            //let ncode = urlencoding::encode(&code);
                            let ncode = code
                                .replace("%", "%25")
                                .replace("<", "%3C")
                                .replace(">", "%3E")
                                .replace(":", "%3A")
                                .replace("\"", "%22")
                                .replace("/", "%2F")
                                .replace("\\", "%5C")
                                .replace("|", "%7C")
                                .replace("?", "%3F")
                                .replace("*", "%2A");

                            qrcode_generator::to_png_to_file(
                                &fsdata,
                                QrCodeEcc::Low,
                                1024,
                                format!("{}{}{}{}", img_path, "/FS-4D-", ncode, ".png"),
                            )
                            .unwrap();
                            match image::SharedImage::load(format!(
                                "{}/FS-4D-{}.png",
                                img_path, ncode
                            )) {
                                Ok(mut nimage) => {
                                    nimage.scale(550, 400, true, true);
                                    iframe.set_label_color(fltk::enums::Color::Red);

                                    iframe.set_image(Some(nimage));
                                    iframe.set_label(
                                        format!(
                                            "FS-4D-{}\n二维码大图已保存到{}文件夹",
                                            code, img_path
                                        )
                                        .as_str(),
                                    );
                                }
                                Err(_) => {}
                            }
                        } else if choice.value() == 5 {
                            let code = code.to_uppercase();

                            let b_code = base64::encode(&code);
                            // println!("{}",b_code);
                            let qrdata = format!(
                "https://peisong.meituan.com/app/wxmpCabinet/roleSelect?mtDeviceId={}",
                b_code
            );
                            //let ncode = urlencoding::encode(&code);
                            let ncode = code
                                .replace("%", "%25")
                                .replace("<", "%3C")
                                .replace(">", "%3E")
                                .replace(":", "%3A")
                                .replace("\"", "%22")
                                .replace("/", "%2F")
                                .replace("\\", "%5C")
                                .replace("|", "%7C")
                                .replace("?", "%3F")
                                .replace("*", "%2A");

                            qrcode_generator::to_png_to_file(
                                qrdata,
                                QrCodeEcc::High,
                                1024,
                                format!("{}/MT-{}.png", img_path, ncode),
                            )
                            .unwrap();
                            match image::SharedImage::load(format!("{}/MT-{}.png", img_path, ncode))
                            {
                                Ok(mut nimage) => {
                                    nimage.scale(550, 400, true, true);
                                    iframe.set_label_color(fltk::enums::Color::Red);

                                    iframe.set_image(Some(nimage));
                                    iframe.set_label(
                                        format!(
                                            "MT-{}\n二维码大图已保存到{}文件夹",
                                            code, img_path
                                        )
                                        .as_str(),
                                    );
                                }
                                Err(_) => {}
                            }
                        } else if choice.value() == 6 {
                            //let ncode = urlencoding::encode(&code);
                            let ncode = code
                                .replace("%", "%25")
                                .replace("<", "%3C")
                                .replace(">", "%3E")
                                .replace(":", "%3A")
                                .replace("\"", "%22")
                                .replace("/", "%2F")
                                .replace("\\", "%5C")
                                .replace("|", "%7C")
                                .replace("?", "%3F")
                                .replace("*", "%2A");

                            qrcode_generator::to_png_to_file(
                                code.clone(),
                                QrCodeEcc::Low,
                                1024,
                                format!("{}{}{}{}", img_path, "/QR-", ncode, ".png"),
                            )
                            .unwrap();
                            match image::SharedImage::load(format!("{}/QR-{}.png", img_path, ncode))
                            {
                                Ok(mut nimage) => {
                                    nimage.scale(550, 400, true, true);
                                    iframe.set_label_color(fltk::enums::Color::Red);

                                    iframe.set_image(Some(nimage));
                                    iframe.set_label(
                                        format!("{}\n二维码大图已保存到{}文件夹", code, img_path)
                                            .as_str(),
                                    );
                                }
                                Err(_) => {}
                            }
                        } else {
                        }

                        app.redraw();
                    }
                }
                _ => println!("hehe {}", r),
            }
        }
    }
}
