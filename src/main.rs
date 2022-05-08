fn image_position(info: &png::OutputInfo, row: u32, column: u32) -> Option<usize> {
    if info.width < column + 1{
        return None
    }else if info.height < row + 1{
        return None
    }
    
    let channel: u32 = if info.color_type == png::ColorType::Grayscale {
        1
    } else if info.color_type == png::ColorType::GrayscaleAlpha {
        2
    } else if info.color_type == png::ColorType::Rgb {
        3
    } else if info.color_type == png::ColorType::Rgba {
        4
    } else{
        return None
    };
    Some((info.width * row * channel + column * channel) as usize)
}

fn poisson_image_editing(tgt: &str, src: &str, dst: &str, repeat: i32){
    let fs: std::fs::File = std::fs::File::open(tgt).unwrap();
    let decoder: png::Decoder<std::fs::File> = png::Decoder::new(fs);
    let mut reader: png::Reader<std::fs::File> = decoder.read_info().unwrap();
    let mut buf: Vec<u8> = vec![0; reader.output_buffer_size()];
    let info: png::OutputInfo = reader.next_frame(&mut buf).unwrap();
    let mut tgt: Vec<f64> = vec![0.0; buf.capacity()];
    for i in 0..buf.capacity(){
        tgt[i] = buf[i] as f64;
    }

    let fs_src: std::fs::File = std::fs::File::open(src).unwrap();
    let decoder_src: png::Decoder<std::fs::File> = png::Decoder::new(fs_src);
    let mut reader_src: png::Reader<std::fs::File> = decoder_src.read_info().unwrap();
    let mut buf_src: Vec<u8> = vec![0; reader_src.output_buffer_size()];
    let _info_src: png::OutputInfo = reader_src.next_frame(&mut buf_src).unwrap();
    let mut src: Vec<f64> = vec![0.0; buf_src.capacity()];

    for i in 0..buf_src.capacity(){
        src[i] = buf_src[i] as f64;
    }
    
    for _n in 1..(repeat){
        for i in 1..(info.height - 1){
            for j in 1..(info.width - 1){
                let p: usize = image_position(&info, i, j).unwrap();
                let t_p: usize = image_position(&info, i - 1, j).unwrap();
                let r_p: usize = image_position(&info, i, j + 1).unwrap();
                let b_p: usize = image_position(&info, i + 1, j).unwrap();
                let l_p: usize = image_position(&info, i, j - 1).unwrap();
                if src[p + 3] == 255.0 {
                    for c in 0..3{
                        tgt[p + c] = src[p + c] + (
                              tgt[t_p + c] - src[t_p + c]
                            + tgt[r_p + c] - src[r_p + c]
                            + tgt[b_p + c] - src[b_p + c]
                            + tgt[l_p + c] - src[l_p + c]
                        ) / 4.0;
                    }
                }
            }
        }
    }

    let mut out_buf: Vec<u8> = vec![0; tgt.capacity()];
    for i in 0..buf_src.capacity(){
        out_buf[i] = tgt[i] as u8;
    }

    let file: std::fs::File = std::fs::File::create(dst).unwrap();
    let w = std::io::BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, info.width, info.height);
    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&out_buf).unwrap();
}

fn main() {
    poisson_image_editing(
        "target.png",
        "source.png",
        "output.png",
        10
    );
}
