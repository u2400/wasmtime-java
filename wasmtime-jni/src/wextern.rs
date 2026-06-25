use crate::errors::{Error, Result};
use crate::{interop, utils};
use jni::objects::JObject;
use jni::JNIEnv;
use wasmtime::{Extern, Func, Global, Memory, Table};

pub fn from_java<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>) -> Result<Extern> {
    let ty = env
        .get_field(&obj, "type", "Lio/github/u2400/wasmtime/Extern$Type;")?
        .l()?;
    let name = utils::enum_name(env, ty)?;
    let extn = match name.as_str() {
        "FUNC" => {
            let fn_obj = env
                .call_method(obj, "func", "()Lio/github/u2400/wasmtime/Func;", &[])?
                .l()?;
            let func = interop::get_inner::<Func>(env, &fn_obj)?;
            Extern::from(func.clone())
        }
        "MEMORY" => {
            let mem_obj = env
                .call_method(
                    obj,
                    "memory",
                    "()Lio/github/u2400/wasmtime/Memory;",
                    &[],
                )?
                .l()?;
            let memory = interop::get_inner::<Memory>(env, &mem_obj)?;
            Extern::from(memory.clone())
        }
        _ => return Err(Error::UnknownEnum(name)),
    };
    Ok(extn)
}

pub fn into_java<'a>(env: &mut JNIEnv<'a>, ext: Extern) -> Result<JObject<'a>> {
    Ok(match ext {
        Extern::Func(func) => {
            let fn_obj = env.new_object(
                "io/github/u2400/wasmtime/Func",
                "(J)V",
                &[interop::into_raw::<Func>(func).into()],
            )?;
            env.call_static_method(
                "io/github/u2400/wasmtime/Extern",
                "fromFunc",
                "(Lio/github/u2400/wasmtime/Func;)Lio/github/u2400/wasmtime/Extern;",
                &[(&fn_obj).into()],
            )?
            .l()?
        }
        Extern::Memory(memory) => {
            let mem_obj = env.new_object(
                "io/github/u2400/wasmtime/Memory",
                "(J)V",
                &[interop::into_raw::<Memory>(memory).into()],
            )?;
            env.call_static_method(
                "io/github/u2400/wasmtime/Extern",
                "fromMemory",
                "(Lio/github/u2400/wasmtime/Memory;)Lio/github/u2400/wasmtime/Extern;",
                &[(&mem_obj).into()],
            )?
            .l()?
        }
        Extern::Table(table) => {
            let table_obj = env.new_object(
                "io/github/u2400/wasmtime/Table",
                "(J)V",
                &[interop::into_raw::<Table>(table).into()],
            )?;
            env.call_static_method(
                "io/github/u2400/wasmtime/Extern",
                "fromTable",
                "(Lio/github/u2400/wasmtime/Table;)Lio/github/u2400/wasmtime/Extern;",
                &[(&table_obj).into()],
            )?
            .l()?
        }
        Extern::Global(global) => {
            let global_obj = env.new_object(
                "io/github/u2400/wasmtime/Global",
                "(J)V",
                &[interop::into_raw::<Global>(global).into()],
            )?;
            env.call_static_method(
                "io/github/u2400/wasmtime/Extern",
                "fromGlobal",
                "(Lio/github/u2400/wasmtime/Global;)Lio/github/u2400/wasmtime/Extern;",
                &[(&global_obj).into()],
            )?
            .l()?
        }
        _ => return Err(Error::NotImplemented),
    })
}

pub fn unknown<'a>(env: &mut JNIEnv<'a>) -> Result<JObject<'a>> {
    Ok(env
        .get_static_field(
            "io/github/u2400/wasmtime/Extern",
            "UNKNOWN",
            "Lio/github/u2400/wasmtime/Extern;",
        )?
        .l()?)
}
