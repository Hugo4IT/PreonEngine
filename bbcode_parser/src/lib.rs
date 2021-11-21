use wgpu_glyph::Text;

enum DecodingStatus {
    Tag(String),
    TagEnd(String),
    VariableName(String),
    VariableValue(String, String),
    Text(String),
}

pub struct BBCodeDecoder<'a> {
    input: String,
    status: DecodingStatus,
    decoded: Vec<Text<'a>>,
}

impl<'a> BBCodeDecoder<'a> {
    fn new(input: String) -> BBCodeDecoder<'a> {
        BBCodeDecoder {
            input,
            status: DecodingStatus::Text(String::new()),
            decoded: Vec::new(),
        }
    }

    fn run(&mut self) {
        for ch in self.input.drain(..) {
            match self.status {
                DecodingStatus::Tag(_) => todo!(),
                DecodingStatus::TagEnd(_) => todo!(),
                DecodingStatus::VariableName(_) => todo!(),
                DecodingStatus::VariableValue(_, _) => todo!(),
                DecodingStatus::Text(ref mut text) =>  {
                    match ch {
                        '[' => self.status = DecodingStatus::Tag(String::new()),
                        _ => text.push(ch),
                    }
                },
            }
        }
    }

    pub fn decode(input: String) {
        Self::new(input).run();
    }
}
