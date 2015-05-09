use std::io;
use gfx;
use ::aux::ReadExt;
use ::chunk::Root;

pub type Success<R> = (gfx::Mesh<R>, gfx::Slice<R>);

/// Parse type character to gfx attribute type
/// using Python packing notation:
/// https://docs.python.org/2/library/struct.html#format-characters
fn parse_type(type_: char, normalized: u8) -> Result<gfx::attrib::Type, ()> {
    use gfx::attrib::Type::*;
    use gfx::attrib::IntSubType::*;
    use gfx::attrib::IntSize::*;
    use gfx::attrib::SignFlag::*;
    use gfx::attrib::FloatSubType::*;
    use gfx::attrib::FloatSize::*;
    Ok(match (type_, normalized) {
        ('b', 0) => Int(Raw, U8, Signed),
        ('B', 0) => Int(Raw, U8, Unsigned),
        ('b', 1) => Int(Normalized, U8, Signed),
        ('B', 1) => Int(Normalized, U8, Unsigned),
        ('h', 0) => Int(Raw, U16, Signed),
        ('H', 0) => Int(Raw, U16, Unsigned),
        ('h', 1) => Int(Normalized, U16, Signed),
        ('H', 1) => Int(Normalized, U16, Unsigned),
        ('l', 0) => Int(Raw, U32, Signed),
        ('L', 0) => Int(Raw, U32, Unsigned),
        ('l', 1) => Int(Normalized, U32, Signed),
        ('L', 1) => Int(Normalized, U32, Unsigned),
        ('f', 0) => Float(Default, F32),
        ('d', 0) => Float(Precision, F64),
        _ => return Err(()),
    })
}

#[derive(Debug)]
pub enum Error {
    Path(io::Error),
    NameNotInCollection,
    Chunk(String),
    Signature(String),
    Topology(String),
    DoubleIndex,
    AttribType(char, u8),
    IndexType(char),
    Stride(u8),
    Other,
}

pub fn load<I: io::Read, R: gfx::Resources, F: gfx::Factory<R>>(
            reader: &mut Root<I>, factory: &mut F)
            -> Result<(String, Success<R>), Error> {
    use gfx::PrimitiveType;
    let mut cmesh = reader.enter();
    if cmesh.get_name() != "mesh"    {
        return Err(Error::Signature(cmesh.get_name().to_string()))
    }
    let mesh_name = cmesh.read_str().to_string();
    let n_vert = cmesh.read_u32();
    info!("\tname: {}, vertices: {}", mesh_name, n_vert);
    let mut slice = gfx::Slice {
        start: 0,
        end: n_vert,
        prim_type: match cmesh.read_str() {
            "1" => PrimitiveType::Point,
            "2" => PrimitiveType::Line,
            "2s"=> PrimitiveType::LineStrip,
            "3" => PrimitiveType::TriangleList,
            "3s"=> PrimitiveType::TriangleStrip,
            "3f"=> PrimitiveType::TriangleFan,
            top => return Err(Error::Topology(top.to_string())),
        },
        kind: gfx::SliceKind::Vertex,
    };
    let mut mesh = gfx::Mesh::new(n_vert);
    while cmesh.has_more() {
        let mut cbuf = cmesh.enter();
        match &slice.kind {
            _ if cbuf.get_name() == "buffer" => {
                let stride = cbuf.read_u8();
                let format_str = cbuf.read_str().to_string();
                debug!("\tBuffer stride: {}, format: {}", stride, format_str);
                let buffer = {
                    let data = cbuf.read_bytes(n_vert * (stride as u32));
                    factory.create_buffer_static_raw(data, gfx::BufferRole::Vertex)
                };
                let mut offset = 0;
                for sub in format_str.as_bytes().chunks(2) {
                    let el_count = sub[0] - ('0' as u8);
                    let type_ = sub[1] as char;
                    let name = cbuf.read_str().to_string();
                    let flags = cbuf.read_u8();
                    debug!("\t\tname: {}, count: {}, type: {}, flags: {}",
                        name, el_count, type_, flags);
                    let normalized = flags & 1;
                    let el_type = match parse_type(type_, normalized) {
                        Ok(t) => t,
                        Err(_) => return Err(Error::AttribType(type_, flags)),
                    };
                    mesh.attributes.push(gfx::Attribute {
                        name: format!("{}{}", super::PREFIX_ATTRIB, name),
                        buffer: buffer.clone(),
                        format: gfx::attrib::Format {
                            elem_count: el_count,
                            elem_type: el_type,
                            offset: offset as gfx::attrib::Offset,
                            stride: stride as gfx::attrib::Stride,
                            instance_rate: 0,
                        },
                    });
                    offset += el_count * el_type.get_size();
                }
                if offset != stride {
                    return Err(Error::Stride(offset));
                }
            },
            &gfx::SliceKind::Vertex if cbuf.get_name() == "index" => {
                let n_ind = cbuf.read_u32();
                let format = cbuf.read_u8() as char;
                debug!("\tIndex format: {}, count: {}", format, n_ind);
                slice.kind = match format {
                    'B' => {
                        let data = cbuf.read_bytes(n_ind * 1);
                        let buf = factory.create_buffer_static_raw(data, gfx::BufferRole::Index);
                        gfx::SliceKind::Index8(gfx::handle::IndexBuffer::from_raw(buf), 0)
                    },
                    'H' => {
                        let data = cbuf.read_bytes(n_ind * 2);
                        let buf = factory.create_buffer_static_raw(data, gfx::BufferRole::Index);
                        gfx::SliceKind::Index16(gfx::handle::IndexBuffer::from_raw(buf), 0)
                    },
                    'L' => {
                        let data = cbuf.read_bytes(n_ind * 4);
                        let buf = factory.create_buffer_static_raw(data, gfx::BufferRole::Index);
                        gfx::SliceKind::Index32(gfx::handle::IndexBuffer::from_raw(buf), 0)
                    },
                    _ => return Err(Error::IndexType(format)),
                };
            },
            _ if cbuf.get_name() == "index" => return Err(Error::DoubleIndex),
            _ => return Err(Error::Chunk(cbuf.get_name().to_string())),
        }
    }
    Ok((mesh_name, (mesh, slice)))
}
