//! Act 1: The Recurrent Bottleneck (Classical NLP)
//!
//! Demonstrates the fundamental limitation of traditional recurrent models (RNNs/LSTMs):
//! sequential recurrence forces entire sentences through a single fixed-size hidden vector,
//! leading to forgotten distant dependencies and blocking GPU parallelism.

use anyhow::Result;
use psychopomp::{
    author::{PlanBuilder, seconds},
    callout::{CalloutActor, CalloutPlan, CalloutSide},
    plan::ScenePlan,
    stage::{StageActor, StagePlan},
    tone::Tone,
};

use crate::{
    CONFIRM, IMPACT, SCENE_DURATION_SECONDS, SEND, TICK, beam, card, chip, footer, header,
    play_bgm, play_sound, play_voice, post, ring, span, stage_anchor,
};

pub fn scene(with_audio: bool) -> Result<ScenePlan> {
    let mut sc = PlanBuilder::new("01-recurrent-bottleneck", seconds(SCENE_DURATION_SECONDS));

    let stage_plan = StagePlan {
        post: post(),
        elements: vec![
            card(
                "token-animal",
                [310.0, 490.0, 0.0],
                [270.0, 115.0],
                "Token 1: [ animal ]",
                &[("subject noun", Tone::Muted)],
                Tone::Plain,
            ),
            card(
                "token-street",
                [710.0, 490.0, 0.0],
                [270.0, 115.0],
                "Token 5: [ street ]",
                &[("location context", Tone::Muted)],
                Tone::Plain,
            ),
            card(
                "cell-rnn",
                [1150.0, 490.0, 0.0],
                [360.0, 125.0],
                "RNN Bottleneck: h_t ∈ ℝᵈ",
                &[("O(N) sequential bottleneck", Tone::Warning)],
                Tone::Warning,
            ),
            card(
                "token-it",
                [1600.0, 490.0, 0.0],
                [270.0, 115.0],
                "Token 8: [ it = ? ]",
                &[("distant reference lost", Tone::Warning)],
                Tone::Accent,
            ),
            ring("pulse", [1150.0, 490.0, 0.0], 195.0, 3.5, Tone::Warning),
            beam("beam-1", "token-animal", "token-street", Tone::Plain),
            beam("beam-2", "token-street", "cell-rnn", Tone::Plain),
            beam("beam-3", "cell-rnn", "token-it", Tone::Warning),
            crate::packet("pkt-1", "beam-1", false, "h₁", Tone::Plain),
            crate::packet("pkt-2", "beam-2", false, "h₂", Tone::Plain),
            crate::packet("pkt-3", "beam-3", false, "h₃", Tone::Warning),
        ],
    };

    let mut s = StageActor::declare(&mut sc, "stage", &stage_plan)?;

    // Camera move: subtle dolly in
    s.channel(&mut sc, "camera.z", -100.0);
    s.to(&mut sc, "camera.z", 0, 0.0, 2.2);

    // Initial appearance of sequential chain
    s.settle_in(&mut sc, "token-animal", seconds(0.3));
    s.settle_in(&mut sc, "token-street", seconds(0.5));
    let rnn_ready = s.settle_in(&mut sc, "cell-rnn", seconds(0.7));
    s.settle_in(&mut sc, "token-it", seconds(0.9));

    if with_audio {
        play_bgm(&mut sc, "bgm-act1", 0.0, -14.0);
        play_voice(&mut sc, "voice-act1", "voice_act1.wav", 8.736, 0.25, 2.0);
        play_sound(&mut sc, "sfx-rnn", CONFIRM, rnn_ready, -14.0);
    }

    // Connect sequential recurrence beams
    s.connect(&mut sc, "beam-1", seconds(1.3), 0.45);
    s.connect(&mut sc, "beam-2", seconds(1.5), 0.45);
    s.connect(&mut sc, "beam-3", seconds(1.7), 0.45);

    if with_audio {
        play_sound(&mut sc, "sfx-connect", TICK, seconds(1.3), -18.0);
    }

    // Pulsing bottleneck ring around the RNN hidden state
    s.channel(&mut sc, "pulse.sweep", 0.0);
    s.to(&mut sc, "pulse.sweep", seconds(1.5), 1.0, 0.9);
    s.to(&mut sc, "pulse.expand", seconds(1.5), 1.0, 1.3);
    s.to(&mut sc, "pulse.opacity", seconds(2.5), 0.0, 0.5);

    // Send recurrence packets along the chain
    let p1 = s.send(&mut sc, "pkt-1", seconds(2.0), 0.55);
    let p2 = s.send(&mut sc, "pkt-2", seconds(2.3), 0.55);
    let p3 = s.send(&mut sc, "pkt-3", seconds(2.7), 0.65);

    // Fade out packets upon arrival to prevent box edge overwrites
    s.to(&mut sc, "pkt-1.opacity", p1, 0.0, 0.15);
    s.to(&mut sc, "pkt-2.opacity", p2, 0.0, 0.15);
    s.to(&mut sc, "pkt-3.opacity", p3, 0.0, 0.15);

    if with_audio {
        play_sound(&mut sc, "sfx-pkt-send", SEND, seconds(2.0), -12.0);
        play_sound(&mut sc, "sfx-bottleneck-hit", IMPACT, p2, -15.0);
        play_sound(&mut sc, "sfx-it-hit", CONFIRM, p3, -15.0);
    }

    // Bottleneck stress reaction
    s.hit(&mut sc, "cell-rnn.flash", p2, 1.0, 0.0);
    s.hit(&mut sc, "cell-rnn.glow", p2, 1.0, 0.0);
    s.jolt(&mut sc, p2, [8.0, -3.0], 0.35);

    s.hit(&mut sc, "token-it.glow", p3, 0.8, 0.0);

    // Callout explaining the RNN bottleneck problem
    let mut callout = CalloutActor::declare(
        &mut sc,
        "note-bottleneck",
        &CalloutPlan::new(
            stage_anchor("cell-rnn", "cell-rnn", CalloutSide::Top),
            vec![
                span("Information Bottleneck: ", Tone::Warning),
                span(
                    "Fixed vector h_t loses distant context as sequence grows.",
                    Tone::Plain,
                ),
            ],
        )
        .side(CalloutSide::Top)
        .reach(70.0)
        .elbow()
        .chip(),
    )?;
    callout.show(&mut sc, seconds(2.0));

    // Headers & footers
    header(&mut sc, "01", "the recurrent bottleneck")?;
    chip(&mut sc, "CLASSICAL NLP")?;
    footer(
        &mut sc,
        "footer",
        &[
            ("Sequential recurrence forces context through ", Tone::Muted),
            ("a fixed-size vector h_t. ", Tone::Warning),
            (
                "Long-range signals vanish and sequential execution blocks parallel GPUs.",
                Tone::Plain,
            ),
        ],
        seconds(2.6),
    )?;

    // Presentation steps for interactive playback
    sc.presentation_step("sequential", "Sequential Recurrence", 0, seconds(2.2));
    sc.presentation_step(
        "bottleneck",
        "Hidden State Bottleneck",
        seconds(2.2),
        seconds(5.5),
    );
    sc.presentation_step(
        "loss",
        "Information Loss & No Parallelism",
        seconds(5.5),
        seconds(9.5),
    );

    Ok(sc.finish()?)
}
