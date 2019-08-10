use failure::Error;
use futures::{lazy, Future, IntoFuture};
use stdweb::{unstable::TryInto, web::TypedArray};

pub(crate) mod primitives;

pub(crate) fn random(bytes: u32) -> impl Future<Item = Vec<u8>, Error = Error> {
    lazy(move || {
        let data: TypedArray<u8> = js! {
            let data = new Uint8Array(@{bytes});
            window.crypto.getRandomValues(data);
            return data;
        }
        .try_into()
        .unwrap();
        Ok(data.into()).into_future()
    })
}