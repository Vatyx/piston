
#![feature(test)]

extern crate test;
extern crate event;

use test::Bencher;
use event::{ Event, AfterRenderArgs, AfterRenderEvent };

#[bench]
fn bench_event_after_render(bencher: &mut Bencher) {
    let e = Event::AfterRender(AfterRenderArgs);
    let args = AfterRenderArgs;
    bencher.iter(|| {
        let _: Option<Event> = AfterRenderEvent::from_after_render_args(&args, &e);
    });
}
