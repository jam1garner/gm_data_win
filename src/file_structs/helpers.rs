use super::PosSlice;

use binrw::{BinRead, BinResult, BinReaderExt, ReadOptions};
use binrw::io::{Cursor, Read, Seek, SeekFrom};
use binrw::io;

pub(crate) struct PosCursor<'a> {
    pos: usize,
    inner: Cursor<&'a [u8]>,
}

impl<'a> From<PosSlice<'a>> for PosCursor<'a> {
    fn from(input: PosSlice<'a>) -> Self {
        let pos = input.pos();
        let inner = Cursor::new(input.1);

        Self { pos, inner }
    }
}

impl<'a> Seek for PosCursor<'a> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match pos {
            SeekFrom::Start(pos) => {
                let pos = pos.checked_sub(self.pos as u64)
                    .ok_or_else(|| io::Error::new(
                        io::ErrorKind::Other,
                        "seek position out of PosCursor range"
                    ))?;

                self.inner.seek(SeekFrom::Start(pos))
                    .map(|pos| pos + (self.pos as u64))
            },
            SeekFrom::Current(rel) => {
                self.inner.seek(SeekFrom::Current(rel))
                    .map(|pos| pos + (self.pos as u64))
            }
            _ => unimplemented!()
        }
    }
}

impl<'a> Read for PosCursor<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

pub(crate) fn ptr_list<R: Read + Seek, T: BinRead<Args = ()>>(
    reader: &mut R,
    options: &ReadOptions,
    _: ()
) -> BinResult<Vec<T>> {
    let count = reader.read_le::<u32>()? as usize;
    let ptrs = reader.read_type_args::<Vec<u32>>(options.endian, binrw::args!{ count })?;

    ptrs.into_iter()
        .map(|pos| -> BinResult<T> {
            reader.seek(SeekFrom::Start(pos as u64))?;
            reader.read_le()
        })
        .collect()
}
