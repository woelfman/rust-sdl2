use std::cast;
use rect::Rect;
use get_error;
use std::ptr;
use std::libc::c_int;

pub mod ll {
    use pixels::ll::SDL_PixelFormat;
    use pixels::ll::SDL_Palette;
    use render::ll::SDL_BlendMode;
    use rwops::ll::SDL_RWops;
    use rect::Rect;
    use std::libc::{c_int, c_void, uint32_t, uint8_t};
    pub type SDL_Rect = Rect;
    pub type SDL_bool = c_int;

    pub type SDL_SurfaceFlag = c_int;

    pub static SDL_SWSURFACE: SDL_SurfaceFlag = 0;
    pub static SDL_PREALLOC: SDL_SurfaceFlag = 0x00000001;
    pub static SDL_RLEACCEL: SDL_SurfaceFlag = 0x00000002;
    pub static SDL_DONTFREE: SDL_SurfaceFlag = 0x00000004;

    //SDL_surface.h
    pub struct SDL_BlitMap;

    pub struct SDL_Surface {
        flags: uint32_t,
        format: *SDL_PixelFormat,
        w: c_int,
        h: c_int,
        pitch: c_int,
        pixels: *c_void,
        userdata: *c_void,
        locked: c_int,
        lock_data: *c_void,
        clip_rect: SDL_Rect,
        map: *SDL_BlitMap,
        refcount: c_int
    }

    externfn!(fn SDL_CreateRGBSurface(flags: uint32_t, width: c_int, height: c_int, depth: c_int, Rmask: uint32_t, Gmask: uint32_t, Bmask: uint32_t, Amask: uint32_t) ->  *SDL_Surface)
    externfn!(fn SDL_CreateRGBSurfaceFrom(pixels: *c_void, width: c_int, height: c_int, depth: c_int, pitch: c_int, Rmask: uint32_t, Gmask: uint32_t, Bmask: uint32_t, Amask: uint32_t) ->  *SDL_Surface)
    externfn!(fn SDL_FreeSurface(surface: *SDL_Surface))
    externfn!(fn SDL_SetSurfacePalette(surface: *SDL_Surface, palette: *SDL_Palette) -> c_int)
    externfn!(fn SDL_LockSurface(surface: *SDL_Surface) -> c_int)
    externfn!(fn SDL_UnlockSurface(surface: *SDL_Surface))
    externfn!(fn SDL_LoadBMP_RW(src: *SDL_RWops, freesrc: c_int) ->  *SDL_Surface)
    externfn!(fn SDL_SaveBMP_RW(surface: *SDL_Surface, dst: *SDL_RWops, freedst: c_int) -> c_int)
    externfn!(fn SDL_SetSurfaceRLE(surface: *SDL_Surface, flag: c_int) -> c_int)
    externfn!(fn SDL_SetColorKey(surface: *SDL_Surface, flag: c_int, key: uint32_t) -> c_int)
    externfn!(fn SDL_GetColorKey(surface: *SDL_Surface, key: *uint32_t) -> c_int)
    externfn!(fn SDL_SetSurfaceColorMod(surface: *SDL_Surface, r: uint8_t, g: uint8_t, b: uint8_t) -> c_int)
    externfn!(fn SDL_GetSurfaceColorMod(surface: *SDL_Surface, r: *uint8_t, g: *uint8_t, b: *uint8_t ) -> c_int)
    externfn!(fn SDL_SetSurfaceAlphaMod(surface: *SDL_Surface, alpha: uint8_t) -> c_int)
    externfn!(fn SDL_GetSurfaceAlphaMod(surface: *SDL_Surface, alpha: *uint8_t ) -> c_int)
    externfn!(fn SDL_SetSurfaceBlendMode(surface: *SDL_Surface, blendMode: SDL_BlendMode) -> c_int)
    externfn!(fn SDL_GetSurfaceBlendMode(surface: *SDL_Surface, blendMode: *SDL_BlendMode) -> c_int)
    externfn!(fn SDL_SetClipRect(surface: *SDL_Surface, rect: *SDL_Rect) ->  SDL_bool)
    externfn!(fn SDL_GetClipRect(surface: *SDL_Surface, rect: *SDL_Rect))
    externfn!(fn SDL_ConvertSurface(src: *SDL_Surface, fmt: *SDL_PixelFormat, flags: uint32_t) ->  *SDL_Surface)
    externfn!(fn SDL_ConvertSurfaceFormat(src: *SDL_Surface, pixel_format: uint32_t, flags: uint32_t) ->  *SDL_Surface)
    externfn!(fn SDL_ConvertPixels(width: c_int, height: c_int, src_format: uint32_t, src: *c_void, src_pitch: c_int, dst_format: uint32_t, dst: *c_void, dst_pitch: c_int) -> c_int)
    externfn!(fn SDL_FillRect(dst: *SDL_Surface, rect: *SDL_Rect, color: uint32_t) -> c_int)
    externfn!(fn SDL_FillRects(dst: *SDL_Surface, rects: *SDL_Rect, count: c_int, color: uint32_t) -> c_int)
    externfn!(fn SDL_UpperBlit(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_LowerBlit(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_SoftStretch(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_UpperBlitScaled(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_LowerBlitScaled(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int)
}

#[deriving(Eq)]
pub enum SurfaceFlag {
    SWSurface = ll::SDL_SWSURFACE as int,
    PreAlloc = ll::SDL_PREALLOC as int,
    RLEAccel = ll::SDL_RLEACCEL as int,
    DontFree = ll::SDL_DONTFREE as int
}

#[deriving(Eq)]
pub struct Surface {
    raw: *ll::SDL_Surface,
    owned: bool
}

impl Drop for Surface {
    fn drop(&self) {
        if self.owned {
            unsafe {
                ll::SDL_FreeSurface(self.raw);
            }
        }
    }
}

impl Surface {
    pub fn new(surface_flags: &[SurfaceFlag], width: int, height: int, bpp: int,
               rmask: u32, gmask: u32, bmask: u32, amask: u32) -> Result<~Surface, ~str> {
        let flags = surface_flags.iter().fold(0u32, |flags, flag| { flags | *flag as u32 });

        unsafe {
            let raw = ll::SDL_CreateRGBSurface(flags, width as c_int, height as c_int, bpp as c_int,
                                               rmask, gmask, bmask, amask);

            if raw == ptr::null() {
                Err(get_error())
            } else {
                Ok(~Surface { raw: raw, owned: true })
            }
        }
    }

    //TODO: From Data
    pub fn get_width(&self) -> u16 {
        unsafe { (*self.raw).w as u16 }
    }

    pub fn get_height(&self) -> u16 {
        unsafe { (*self.raw).h as u16 }
    }

    pub fn get_size(&self) -> (u16, u16) {
        (self.get_width(), self.get_height())
    }

    pub fn get_rect(&self) -> Rect {
        Rect {
            x: 0,
            y: 0,
            w: self.get_width() as i32,
            h: self.get_height() as i32
        }
    }
    //externfn!(fn SDL_SetSurfacePalette(surface: *SDL_Surface, palette: *SDL_Palette) -> c_int)


    pub fn lock(&self) -> bool {
        unsafe { ll::SDL_LockSurface(self.raw) == 0 }
    }

    /// Locks a surface so that the pixels can be directly accessed safely.
    pub fn with_lock<R>(&self, f: &fn(pixels: &mut [u8]) -> R) -> R {
        unsafe {
            if ll::SDL_LockSurface(self.raw) != 0 { fail!(~"could not lock surface"); }
            let len = (*self.raw).pitch as uint * ((*self.raw).h as uint);
            let pixels: &mut [u8] = cast::transmute(((*self.raw).pixels, len));
            let rv = f(pixels);
            ll::SDL_UnlockSurface(self.raw);
            rv
        }
    }

    pub fn unlock(&self) {
        unsafe { ll::SDL_UnlockSurface(self.raw); }
    }

    /*externfn!(fn SDL_LoadBMP_RW(src: *SDL_RWops, freesrc: c_int) ->  *SDL_Surface)
    externfn!(fn SDL_SaveBMP_RW(surface: *SDL_Surface, dst: *SDL_RWops, freedst: c_int) -> c_int)
    externfn!(fn SDL_SetSurfaceRLE(surface: *SDL_Surface, flag: c_int) -> c_int)
    externfn!(fn SDL_SetColorKey(surface: *SDL_Surface, flag: c_int, key: uint32_t) -> c_int)
    externfn!(fn SDL_GetColorKey(surface: *SDL_Surface, key: *uint32_t) -> c_int)
    externfn!(fn SDL_SetSurfaceColorMod(surface: *SDL_Surface, r: uint8_t, g: uint8_t, b: uint8_t) -> c_int)
    externfn!(fn SDL_GetSurfaceColorMod(surface: *SDL_Surface, r: *uint8_t, g: *uint8_t, b: *uint8_t ) -> c_int)
    externfn!(fn SDL_SetSurfaceAlphaMod(surface: *SDL_Surface, alpha: uint8_t) -> c_int)
    externfn!(fn SDL_GetSurfaceAlphaMod(surface: *SDL_Surface, alpha: *uint8_t ) -> c_int)
    externfn!(fn SDL_SetSurfaceBlendMode(surface: *SDL_Surface, blendMode: SDL_BlendMode) -> c_int)
    externfn!(fn SDL_GetSurfaceBlendMode(surface: *SDL_Surface, blendMode: *SDL_BlendMode) -> c_int)
    externfn!(fn SDL_SetClipRect(surface: *SDL_Surface, rect: *SDL_Rect) ->  SDL_bool)
    externfn!(fn SDL_GetClipRect(surface: *SDL_Surface, rect: *SDL_Rect))
    externfn!(fn SDL_ConvertSurface(src: *SDL_Surface, fmt: *SDL_PixelFormat, flags: uint32_t) ->  *SDL_Surface)
    externfn!(fn SDL_ConvertSurfaceFormat(src: *SDL_Surface, pixel_format: uint32_t, flags: uint32_t) ->  *SDL_Surface)
    externfn!(fn SDL_ConvertPixels(width: c_int, height: c_int, src_format: uint32_t, src: *c_void, src_pitch: c_int, dst_format: uint32_t, dst: *c_void, dst_pitch: c_int) -> c_int)
    externfn!(fn SDL_FillRect(dst: *SDL_Surface, rect: *SDL_Rect, color: uint32_t) -> c_int)
    externfn!(fn SDL_FillRects(dst: *SDL_Surface, rects: *SDL_Rect, count: c_int, color: uint32_t) -> c_int)
    externfn!(fn SDL_UpperBlit(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_LowerBlit(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_SoftStretch(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_UpperBlitScaled(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int)
    externfn!(fn SDL_LowerBlitScaled(src: *SDL_Surface, srcrect: *SDL_Rect, dst: *SDL_Surface, dstrect: *SDL_Rect) -> c_int)*/
}
