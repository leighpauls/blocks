use quicksilver::{
    graphics::{Font, Image},
    Error, Future,
};

pub struct Images {
    pub empty_mino: Image,
    pub i_mino: Image,
    pub o_mino: Image,
    pub j_mino: Image,
    pub l_mino: Image,
    pub s_mino: Image,
    pub z_mino: Image,
    pub t_mino: Image,
}

pub struct Resources {
    pub font: Font,
    pub images: Images,
}

pub type ResourceFuture = Future<Item = Resources, Error = Error>;

pub fn load_resources() -> impl Future<Item = Resources, Error = Error> {
    Font::load("Roboto-Medium.ttf")
        .join(Image::load("empty_mino.png"))
        .join(Image::load("i_mino.png"))
        .join(Image::load("o_mino.png"))
        .join(Image::load("j_mino.png"))
        .join(Image::load("l_mino.png"))
        .join(Image::load("s_mino.png"))
        .join(Image::load("z_mino.png"))
        .join(Image::load("t_mino.png"))
        .and_then(|big_future| {
            let ((((((((font, empty), i), o), j), l), s), z), t) = big_future;
            Ok(Resources {
                font: font,
                images: Images {
                    empty_mino: empty,
                    i_mino: i,
                    o_mino: o,
                    j_mino: j,
                    l_mino: l,
                    s_mino: s,
                    z_mino: z,
                    t_mino: t,
                },
            })
        })
}
