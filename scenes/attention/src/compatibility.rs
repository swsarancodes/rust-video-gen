//! Act 3: The Trinity of Projections (Q, K, V)
//!
//! Illustrates how each token projects into three distinct semantic roles:
//! Query (what am I looking for?), Key (what information do I advertise?),
//! and Value (what is my actual semantic content to pass along?).

use anyhow::Result;
use psychopomp::{
    author::{PlanBuilder, seconds},
    callout::{CalloutActor, CalloutPlan, CalloutSide},
    plan::ScenePlan,
    stage::{StageActor, StagePlan},
    tone::Tone,
};

use crate::{
    CONFIRM, SCENE_DURATION_SECONDS, SEND, SUCCESS, TICK, bent, card, chip, footer, header,
    play_bgm, play_sound, play_voice, post, span, stage_anchor,
};

pub fn scene(with_audio: bool) -> Result<ScenePlan> {
    let mut sc = PlanBuilder::new("03-projections-qkv", seconds(SCENE_DURATION_SECONDS));

    let stage_plan = StagePlan {
        post: post(),
        elements: vec![
            card(
                "card-x",
                [960.0, 360.0, 0.0],
                [440.0, 115.0],
                "Input Vector: X_it ∈ ℝᵈ",
                &[("token embedding + position encoding", Tone::Muted)],
                Tone::Plain,
            ),
            card(
                "card-q",
                [420.0, 700.0, 0.0],
                [340.0, 130.0],
                "Query (Q)",
                &[("Q = X · W_Q  |  What I seek", Tone::Accent)],
                Tone::Accent,
            ),
            card(
                "card-k",
                [960.0, 700.0, 0.0],
                [340.0, 130.0],
                "Key (K)",
                &[("K = X · W_K  |  What I offer", Tone::Warning)],
                Tone::Warning,
            ),
            card(
                "card-v",
                [1500.0, 700.0, 0.0],
                [340.0, 130.0],
                "Value (V)",
                &[("V = X · W_V  |  My content", Tone::Success)],
                Tone::Success,
            ),
            bent("beam-q", "card-x", "card-q", -26.0, Tone::Accent),
            bent("beam-k", "card-x", "card-k", 0.0, Tone::Warning),
            bent("beam-v", "card-x", "card-v", 26.0, Tone::Success),
            crate::packet("pkt-q", "beam-q", false, "× W_Q", Tone::Accent),
            crate::packet("pkt-k", "beam-k", false, "× W_K", Tone::Warning),
            crate::packet("pkt-v", "beam-v", false, "× W_V", Tone::Success),
        ],
    };

    let mut s = StageActor::declare(&mut sc, "stage", &stage_plan)?;

    // Camera move
    s.channel(&mut sc, "camera.z", -85.0);
    s.to(&mut sc, "camera.z", 0, 0.0, 2.2);

    // Settle input vector
    let x_ready = s.settle_in(&mut sc, "card-x", seconds(0.3));

    if with_audio {
        play_bgm(&mut sc, "bgm-act3", 17.8, -14.0);
        play_voice(&mut sc, "voice-act3", "voice_act3.wav", 7.560, 0.25, 2.0);
        play_sound(&mut sc, "sfx-x", CONFIRM, x_ready, -14.0);
    }

    // Settle projections
    let _q_ready = s.settle_in(&mut sc, "card-q", seconds(0.75));
    let _k_ready = s.settle_in(&mut sc, "card-k", seconds(0.9));
    let _v_ready = s.settle_in(&mut sc, "card-v", seconds(1.05));

    // Connect linear transformation beams
    s.connect(&mut sc, "beam-q", seconds(1.3), 0.45);
    s.connect(&mut sc, "beam-k", seconds(1.4), 0.45);
    s.connect(&mut sc, "beam-v", seconds(1.5), 0.45);

    if with_audio {
        play_sound(&mut sc, "sfx-connect", TICK, seconds(1.3), -18.0);
    }

    // Send projection packets down into Q, K, V
    let pq = s.send(&mut sc, "pkt-q", seconds(2.1), 0.65);
    let pk = s.send(&mut sc, "pkt-k", seconds(2.2), 0.65);
    let pv = s.send(&mut sc, "pkt-v", seconds(2.3), 0.65);

    // Fade out packets upon arrival to avoid overwriting card borders
    s.to(&mut sc, "pkt-q.opacity", pq, 0.0, 0.15);
    s.to(&mut sc, "pkt-k.opacity", pk, 0.0, 0.15);
    s.to(&mut sc, "pkt-v.opacity", pv, 0.0, 0.15);

    if with_audio {
        play_sound(&mut sc, "sfx-send-qkv", SEND, seconds(2.1), -12.0);
        play_sound(&mut sc, "sfx-land-qkv", SUCCESS, pq, -15.0);
    }

    s.hit(&mut sc, "card-q.flash", pq, 1.0, 0.0);
    s.hit(&mut sc, "card-k.flash", pk, 1.0, 0.0);
    s.hit(&mut sc, "card-v.flash", pv, 1.0, 0.0);

    // Callout anchored to top of card-x
    let mut callout = CalloutActor::declare(
        &mut sc,
        "note-q",
        &CalloutPlan::new(
            stage_anchor("card-x", "card-x", CalloutSide::Top),
            vec![
                span("Linear Projections: ", Tone::Accent),
                span(
                    "Learned matrices decouple what a token seeks from what it transmits.",
                    Tone::Plain,
                ),
            ],
        )
        .side(CalloutSide::Top)
        .reach(65.0)
        .elbow()
        .chip(),
    )?;
    callout.show(&mut sc, seconds(2.0));

    // Headers & footers
    header(&mut sc, "03", "the trinity: query, key, value")?;
    chip(&mut sc, "PROJECTIONS")?;
    footer(
        &mut sc,
        "footer",
        &[
            ("Learned linear projections ", Tone::Muted),
            ("W_Q, W_K, W_V ", Tone::Accent),
            (
                "specialize each token into three distinct search, match, and payload spaces.",
                Tone::Plain,
            ),
        ],
        seconds(2.6),
    )?;

    // Presentation steps
    sc.presentation_step("input", "Input Embedding X", 0, seconds(2.2));
    sc.presentation_step(
        "projections",
        "Three Projections Q, K, V",
        seconds(2.2),
        seconds(5.5),
    );
    sc.presentation_step(
        "roles",
        "Search, Identity, and Payload",
        seconds(5.5),
        seconds(9.5),
    );

    Ok(sc.finish()?)
}
