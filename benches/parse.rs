#![feature(test)]

extern crate pico_sys as pico;
extern crate httparse;

extern crate test;

const REQ_SHORT: &'static [u8] = b"\
GET / HTTP/1.0\r\n\
Host: example.com\r\n\
Cookie: session=60; user_id=1\r\n\r\n";

const REQ: &'static [u8] = b"\
GET /wp-content/uploads/2010/03/hello-kitty-darth-vader-pink.jpg HTTP/1.1\r\n\
Host: www.kittyhell.com\r\n\
User-Agent: Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10.6; ja-JP-mac; rv:1.9.2.3) Gecko/20100401 Firefox/3.6.3 Pathtraq/0.9\r\n\
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n\
Accept-Language: ja,en-us;q=0.7,en;q=0.3\r\n\
Accept-Encoding: gzip,deflate\r\n\
Accept-Charset: Shift_JIS,utf-8;q=0.7,*;q=0.7\r\n\
Keep-Alive: 115\r\n\
Connection: keep-alive\r\n\
Cookie: wp_ozh_wsa_visits=2; wp_ozh_wsa_visit_lasttime=xxxxxxxxxx; __utma=xxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.x; __utmz=xxxxxxxxx.xxxxxxxxxx.x.x.utmccn=(referral)|utmcsr=reader.livedoor.com|utmcct=/reader/|utmcmd=referral|padding=under256\r\n\r\n";

#[bench]
fn bench_httparse(b: &mut test::Bencher) {
    let mut headers = [httparse::Header{ name: "", value: &[] }; 16];
    let mut req = httparse::Request::new(&mut headers);
    b.iter(|| {
        assert_eq!(req.parse(REQ).unwrap(), httparse::Status::Complete(REQ.len()));
    });
    b.bytes = REQ.len() as u64;
}

#[bench]
fn bench_httparse_short(b: &mut test::Bencher) {
    let mut headers = [httparse::Header{ name: "", value: &[] }; 16];
    let mut req = httparse::Request::new(&mut headers);
    b.iter(|| {
        assert_eq!(req.parse(REQ_SHORT).unwrap(), httparse::Status::Complete(REQ_SHORT.len()));
    });
    b.bytes = REQ_SHORT.len() as u64;
}
