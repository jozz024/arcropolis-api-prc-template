#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

use prc::hash40::{Hash40, to_hash40};
use arcropolis_api::*;
use prc::*;

#[arc_callback]
fn edit_ui_chara_db(hash: u64, mut data: &mut [u8]) -> Option<usize> {
    load_original_file(hash, &mut data);
    // with the param data ready,
    let mut reader = std::io::Cursor::new(&mut data);
    let mut root = prc::read_stream(&mut reader).unwrap();

    // enter the first and only node of the file ("db_root")
    let (db_root_hash, db_root) = &mut root.0[0];
    assert_eq!(*db_root_hash, to_hash40("db_root"));

    let db_root_list = db_root.try_into_mut::<ParamList>().unwrap();

    // iterate the list to find the param with mario's data
    // we could go to the exact index, but this is subject to change across game updates.
    let mario = db_root_list.0.iter_mut().find(|param| {
        let ui_chara_struct = param.try_into_ref::<ParamStruct>().unwrap();

        // we assume ui_chara_id will always be the first param.
        // given the file, this is a safe assumption, but there are
        // more fool-proof ways of searching for the right node.
        let (_, ui_chara_id) = &ui_chara_struct.0[0];
        let ui_chara_hash = ui_chara_id.try_into_ref::<Hash40>().unwrap();

        // check to make sure it's mario
        *ui_chara_hash == to_hash40("ui_chara_mario")
    }).unwrap().try_into_mut::<ParamStruct>().unwrap();

    // now we have mario's data, we can convert to a dictionary to gain faster access
    // to arbitrary keys, but since we only want to change 1 param, we'll just iterate
    mario.0.iter_mut().for_each(|(hash, param)| {
        if *hash == to_hash40("ui_series_id") {
            *param.try_into_mut::<Hash40>().unwrap() = to_hash40("ui_series_fatalfury");
        }
    });
    let mut writer = std::io::Cursor::new(data);
    write_stream(&mut writer, &root).unwrap();
    return Some(writer.position() as usize);
}

const MAX_FILE_SIZE: usize = 0xFFFF;

#[skyline::main(name = "runtime_prc_editing_template")]
fn main() {
    edit_ui_chara_db::install("ui/param/database/ui_chara_db.prc", MAX_FILE_SIZE);
}