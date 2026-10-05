//! Act 5: The Transformer Architecture (Foundation Models)
//!
//! Concludes the explainer: Multi-Head Attention runs parallel heads across diverse
//! linguistic relationships, wrapped in Residual Connections and Feed-Forward Networks
//! to build the complete Transformer layer powering modern Large Language Models.

use anyhow::Result;
use psychopomp::{
    author::{PlanBuilder, seconds},
    callout::{CalloutActor, CalloutPlan, CalloutSide},
    plan::ScenePlan,
    stage::{StageActor, StagePlan},
    tone::Tone,
};

use crate::{
    BLOOM, CONFIRM, SCENE_DURATION_SECONDS, SUCCESS, TICK, bent, card, chip, footer, header, orb,
    orb_in, play_bgm, play_sound, play_voice, post, ring, span, stage_anchor,
};

pub fn scene(with_audio: bool) -> Result<ScenePlan> {
    let mut sc = PlanBuilder::new(
        "05-transformer-architecture",
        seconds(SCENE_DURATION_SECONDS),
    );

    let stage_plan = StagePlan {
        post: post(),
        elements: vec![
            card(
                "head-1",
                [330.0, 340.0, 0.0],
                [290.0, 105.0],
                "Head 1: Coreference",
                &[("pronoun → antecedent", Tone::Accent)],
                Tone::Accent,
            ),
            card(
                "head-2",
                [330.0, 530.0, 0.0],
                [290.0, 105.0],
                "Head 2: Syntactic Link",
                &[("grammatical agreement", Tone::Plain)],
                Tone::Plain,
            ),
            card(
                "head-3",
                [330.0, 720.0, 0.0],
                [290.0, 105.0],
                "Head 3: Causal Context",
                &[("because → tired reason", Tone::Warning)],
                Tone::Warning,
            ),
            card(
                "card-transformer",
                [950.0, 530.0, 0.0],
                [440.0, 130.0],
                "Transformer Block (Layer ℓ)",
                &[("Multi-Head + Residuals + FFN", Tone::Success)],
                Tone::Success,
            ),
            orb("orb-core", [950.0, 310.0, 0.0], 55.0, 480, Tone::Accent),
            ring("aura", [950.0, 530.0, 0.0], 240.0, 3.5, Tone::Success),
            card(
                "card-stack",
                [1570.0, 530.0, 0.0],
                [350.0, 130.0],
                "Deep Stack (N × Layers)",
                &[("Powers GPT, Claude, Gemini & LLaMA", Tone::Plain)],
                Tone::Plain,
            ),
            bent("beam-h1", "head-1", "card-transformer", 15.0, Tone::Accent),
            bent("beam-h2", "head-2", "card-transformer", 0.0, Tone::Plain),
            bent(
                "beam-h3",
                "head-3",
                "card-transformer",
                -15.0,
                Tone::Warning,
            ),
            bent(
                "beam-stack",
                "card-transformer",
                "card-stack",
                0.0,
                Tone::Success,
            ),
            crate::packet("pkt-h1", "beam-h1", false, "H₁", Tone::Accent),
            crate::packet("pkt-h2", "beam-h2", false, "H₂", Tone::Plain),
            crate::packet("pkt-h3", "beam-h3", false, "H₃", Tone::Warning),
            crate::packet("pkt-stack", "beam-stack", false, "Z_ℓ", Tone::Success),
        ],
    };

    let mut s = StageActor::declare(&mut sc, "stage", &stage_plan)?;

    // Camera: begins close and dollies back smoothly to reveal full multi-head architecture
    s.channel(&mut sc, "camera.z", -110.0);
    s.to(&mut sc, "camera.z", 0, 0.0, 2.2);

    // Orb entrance & central transformer card
    orb_in(&mut s, &mut sc, "orb-core", seconds(0.3));
    let t_ready = s.settle_in(&mut sc, "card-transformer", seconds(0.55));

    if with_audio {
        play_bgm(&mut sc, "bgm-act5", 35.6, -14.0);
        play_voice(&mut sc, "voice-act5", "voice_act5.wav", 8.184, 0.25, 2.0);
        play_sound(&mut sc, "sfx-orb-bloom", BLOOM, seconds(0.3), -12.0);
        play_sound(&mut sc, "sfx-trans", CONFIRM, t_ready, -14.0);
    }

    // Settle multi-head cards
    s.settle_in(&mut sc, "head-1", seconds(0.8));
    s.settle_in(&mut sc, "head-2", seconds(0.95));
    s.settle_in(&mut sc, "head-3", seconds(1.1));
    s.settle_in(&mut sc, "card-stack", seconds(1.3));

    // Connect wires from all heads into the Transformer block
    s.connect(&mut sc, "beam-h1", seconds(1.6), 0.4);
    s.connect(&mut sc, "beam-h2", seconds(1.7), 0.4);
    s.connect(&mut sc, "beam-h3", seconds(1.8), 0.4);
    s.connect(&mut sc, "beam-stack", seconds(2.1), 0.45);

    if with_audio {
        play_sound(&mut sc, "sfx-connect", TICK, seconds(1.6), -18.0);
    }

    // Fire packets from heads into Transformer block
    let p1 = s.send(&mut sc, "pkt-h1", seconds(2.3), 0.55);
    let p2 = s.send(&mut sc, "pkt-h2", seconds(2.4), 0.55);
    let p3 = s.send(&mut sc, "pkt-h3", seconds(2.5), 0.55);

    // Fade out packets upon arrival
    s.to(&mut sc, "pkt-h1.opacity", p1, 0.0, 0.15);
    s.to(&mut sc, "pkt-h2.opacity", p2, 0.0, 0.15);
    s.to(&mut sc, "pkt-h3.opacity", p3, 0.0, 0.15);

    if with_audio {
        play_sound(&mut sc, "sfx-head-arrive", SUCCESS, p1, -14.0);
    }

    // Pulse the central block and orb with light on arrival
    s.hit(&mut sc, "card-transformer.flash", p1, 1.0, 0.0);
    s.hit(&mut sc, "card-transformer.glow", p1, 1.0, 0.0);
    s.hit(&mut sc, "orb-core.pulse", p1, 1.0, 0.0);

    // Stream consolidated representation into Deep Stack
    let ps = s.send(&mut sc, "pkt-stack", p1 + seconds(0.4), 0.65);
    // Crucial: fade out pkt-stack upon landing so it never overlaps the Deep Stack card text
    s.to(&mut sc, "pkt-stack.opacity", ps, 0.0, 0.15);

    s.hit(&mut sc, "card-stack.flash", ps, 1.0, 0.0);
    s.hit(&mut sc, "card-stack.glow", ps, 1.0, 0.0);

    if with_audio {
        play_sound(&mut sc, "sfx-stack", CONFIRM, ps, -14.0);
    }

    // Expanding success ring around the block
    s.channel(&mut sc, "aura.sweep", 0.0);
    s.to(&mut sc, "aura.sweep", p1, 1.0, 0.6);
    s.to(&mut sc, "aura.expand", p1, 1.0, 1.0);
    s.to(&mut sc, "aura.opacity", p1 + seconds(0.6), 0.0, 0.4);

    // Callout anchored cleanly above and to the right of Deep Stack card
    let mut callout = CalloutActor::declare(
        &mut sc,
        "note-foundation",
        &CalloutPlan::new(
            stage_anchor("card-stack", "card-stack", CalloutSide::TopRight),
            vec![
                span("Universal Foundation: ", Tone::Success),
                span("Powers all modern large language models.", Tone::Plain),
            ],
        )
        .side(CalloutSide::TopRight)
        .reach(55.0)
        .elbow()
        .chip(),
    )?;
    callout.show(&mut sc, seconds(2.0));

    // Headers & footers
    header(&mut sc, "05", "the transformer architecture")?;
    chip(&mut sc, "FOUNDATION MODELS")?;
    footer(
        &mut sc,
        "footer",
        &[
            (
                "Multi-head attention combines with residual connections and feed-forward layers ",
                Tone::Muted,
            ),
            ("into the Transformer architecture: ", Tone::Accent),
            (
                "the foundation powering modern large language models.",
                Tone::Plain,
            ),
        ],
        seconds(2.6),
    )?;

    // Presentation steps
    sc.presentation_step("multihead", "Multi-Head Attention", 0, seconds(2.2));
    sc.presentation_step(
        "transformer",
        "Transformer Layer Block",
        seconds(2.2),
        seconds(5.5),
    );
    sc.presentation_step(
        "stack",
        "Foundation Models & LLM Stack",
        seconds(5.5),
        seconds(9.5),
    );

    Ok(sc.finish()?)
}
