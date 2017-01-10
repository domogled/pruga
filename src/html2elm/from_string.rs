
use html2elm::elm_sink::ElmSink;
use html5ever::tokenizer::buffer_queue::BufferQueue;

use html5ever::tokenizer::{Tokenizer, TokenizerOpts};
use tendril::SliceExt;

pub fn from_string(source: String) -> Result<String, String> {

    let mut input = BufferQueue::new();

    // input.push_back(chunk.try_reinterpret().unwrap());
    input.push_back(source.to_tendril());

    let sink = ElmSink::new(4); // { in_char_run: false };

    let mut tok = Tokenizer::new(sink, TokenizerOpts { profile: true, ..Default::default() });
    tok.feed(&mut input);

    tok.end();
    // sink.is_char(false);

    Ok(tok.unwrap().generate_code())
}