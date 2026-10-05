//! Act 2: The Attention Breakthrough (Parallel Routing)
//!
//! Visualizes how Attention breaks the sequential bottleneck:
//! instead of passing signals down a chain, Attention creates direct O(1) random-access
//! connections between every pair of tokens in the sequence simultaneously.

use anyhow::Result;
use psychopomp::{
    author::{PlanBuilder, seconds},
    callout::{CalloutActor, CalloutPlan, CalloutSide},
    plan::ScenePlan,
    stage::{StageActor, StagePlan},
    tone::Tone,
};

use crate::{
    CONFIRM, IMPACT, SCENE_DURATION_SECONDS, SEND, TICK, bent, card, chip, footer, header,
    play_bgm, play_sound, play_voice, post, ring, span, stage_anchor,
};

pub fn scene(with_audio: bool) -> Result<ScenePlan> {
    let mut sc = PlanBuilder::new("02-attention-breakthrough", seconds(SCENE_DURATION_SECONDS));

    let stage_plan = StagePlan {
        post: post(),
        elements: vec![
            card(
                "token-animal",
                [420.0, 370.0, 0.0],
                [310.0, 115.0],
                "Token: [ animal ]",
                &[("living antecedent", Tone::Success)],
                Tone::Success,
            ),
            card(
                "token-street",
                [960.0, 370.0, 0.0],
                [310.0, 115.0],
                "Token: [ street ]",
                &[("inanimate location", Tone::Muted)],
                Tone::Plain,
            ),
            card(
                "token-tired",
                [1500.0, 370.0, 0.0],
                [310.0, 115.0],
                "Token: [ tired ]",
                &[("causal adjective", Tone::Warning)],
                Tone::Warning,
            ),
            card(
                "token-it",
                [960.0, 720.0, 0.0],
                [380.0, 130.0],
                "Target Token: [ it ]",
                &[("random-access query across all tokens", Tone::Accent)],
                Tone::Accent,
            ),
            ring("aura-it", [960.0, 720.0, 0.0], 210.0, 3.5, Tone::Accent),
            bent(
                "beam-animal",
                "token-it",
                "token-animal",
                -24.0,
                Tone::Success,
            ),
            bent("beam-street", "token-it", "token-street", 0.0, Tone::Plain),
            bent("beam-tired", "token-it", "token-tired", 24.0, Tone::Warning),
            crate::packet(
                "pkt-animal",
                "beam-animal",
                false,
                "O(1) hop",
                Tone::Success,
            ),
            crate::packet("pkt-street", "beam-street", false, "O(1) hop", Tone::Plain),
            crate::packet("pkt-tired", "beam-tired", false, "O(1) hop", Tone::Warning),
        ],
    };

    let mut s = StageActor::declare(&mut sc, "stage", &stage_plan)?;

    // Camera move
    s.channel(&mut sc, "camera.z", -85.0);
    s.to(&mut sc, "camera.z", 0, 0.0, 2.2);

    // Initial appearance of parallel tokens
    s.settle_in(&mut sc, "token-animal", seconds(0.3));
    s.settle_in(&mut sc, "token-street", seconds(0.45));
    s.settle_in(&mut sc, "token-tired", seconds(0.6));
    let it_ready = s.settle_in(&mut sc, "token-it", seconds(0.8));

    if with_audio {
        play_bgm(&mut sc, "bgm-act2", 8.9, -14.0);
        play_voice(&mut sc, "voice-act2", "voice_act2.wav", 8.040, 0.25, 2.0);
        play_sound(&mut sc, "sfx-it", CONFIRM, it_ready, -14.0);
    }

    // Connect direct random-access highway beams simultaneously
    s.connect(&mut sc, "beam-animal", seconds(1.3), 0.45);
    s.connect(&mut sc, "beam-street", seconds(1.4), 0.45);
    s.connect(&mut sc, "beam-tired", seconds(1.5), 0.45);

    if with_audio {
        play_sound(&mut sc, "sfx-connect", TICK, seconds(1.3), -18.0);
    }

    // Radiant expanding pulse from "it"
    s.channel(&mut sc, "aura-it.sweep", 0.0);
    s.to(&mut sc, "aura-it.sweep", seconds(1.5), 1.0, 0.8);
    s.to(&mut sc, "aura-it.expand", seconds(1.5), 1.0, 1.2);
    s.to(&mut sc, "aura-it.opacity", seconds(2.5), 0.0, 0.4);

    // Fire parallel attention probe packets simultaneously
    let pa = s.send(&mut sc, "pkt-animal", seconds(2.1), 0.6);
    let ps = s.send(&mut sc, "pkt-street", seconds(2.1), 0.6);
    let pt = s.send(&mut sc, "pkt-tired", seconds(2.1), 0.6);

    // Fade out packets upon arrival to prevent box edge overwrites
    s.to(&mut sc, "pkt-animal.opacity", pa, 0.0, 0.15);
    s.to(&mut sc, "pkt-street.opacity", ps, 0.0, 0.15);
    s.to(&mut sc, "pkt-tired.opacity", pt, 0.0, 0.15);

    if with_audio {
        play_sound(&mut sc, "sfx-parallel-send", SEND, seconds(2.1), -12.0);
        play_sound(&mut sc, "sfx-parallel-hit", IMPACT, pa, -14.0);
    }

    s.hit(&mut sc, "token-animal.flash", pa, 1.0, 0.0);
    s.hit(&mut sc, "token-animal.glow", pa, 1.0, 0.0);
    s.hit(&mut sc, "token-tired.glow", pa, 0.7, 0.0);

    // Callout explaining O(1) direct addressing
    let mut callout = CalloutActor::declare(
        &mut sc,
        "note-routing",
        &CalloutPlan::new(
            stage_anchor("token-animal", "token-animal", CalloutSide::Top),
            vec![
                span("Direct O(1) Routing: ", Tone::Success),
                span(
                    "Tokens examine the entire sequence in parallel without recurrent decay.",
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
    header(&mut sc, "02", "the attention breakthrough")?;
    chip(&mut sc, "PARALLEL ROUTING")?;
    footer(
        &mut sc,
        "footer",
        &[
            (
                "Attention eliminates recurrence. Every position examines all other tokens ",
                Tone::Muted,
            ),
            ("simultaneously in parallel ", Tone::Accent),
            ("with zero information loss over distance.", Tone::Plain),
        ],
        seconds(2.6),
    )?;

    // Presentation steps
    sc.presentation_step("parallel", "Parallel Context Tokens", 0, seconds(2.2));
    sc.presentation_step(
        "routing",
        "Direct O(1) Highways",
        seconds(2.2),
        seconds(5.4),
    );
    sc.presentation_step(
        "scalability",
        "GPU Parallelism & Scaling",
        seconds(5.4),
        seconds(9.5),
    );

    Ok(sc.finish()?)
}
