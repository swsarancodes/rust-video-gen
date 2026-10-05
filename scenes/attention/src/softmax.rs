//! Act 4: Scaled Dot-Product & Softmax Routing
//!
//! Demonstrates how the Query computes directional alignment with all Keys,
//! scales by 1/√d_k to prevent saturation, and normalizes via Softmax into
//! an attention weight distribution that routes semantic Values.

use anyhow::Result;
use psychopomp::{
    author::{PlanBuilder, seconds},
    callout::{CalloutActor, CalloutPlan, CalloutSide},
    plan::ScenePlan,
    stage::{StageActor, StagePlan},
    tone::Tone,
};

use crate::{
    BLOOM, CONFIRM, IMPACT, SCENE_DURATION_SECONDS, SEND, TICK, bent, card, chip, footer, header,
    play_bgm, play_sound, play_voice, post, span, stage_anchor,
};

pub fn scene(with_audio: bool) -> Result<ScenePlan> {
    let mut sc = PlanBuilder::new(
        "04-scaled-dot-product-softmax",
        seconds(SCENE_DURATION_SECONDS),
    );

    let stage_plan = StagePlan {
        post: post(),
        elements: vec![
            card(
                "card-q",
                [280.0, 540.0, 0.0],
                [270.0, 120.0],
                "Query: Q [it]",
                &[("vector seeking subject noun", Tone::Accent)],
                Tone::Accent,
            ),
            card(
                "card-scale",
                [660.0, 540.0, 0.0],
                [270.0, 115.0],
                "Scaling: 1 / √dₖ",
                &[("dₖ = 64  →  scale = 1/8", Tone::Plain)],
                Tone::Plain,
            ),
            card(
                "card-v-animal",
                [1380.0, 330.0, 0.0],
                [450.0, 110.0],
                "Key / Value: [ animal ]",
                &[("α = 88%  |  Dominant payload", Tone::Success)],
                Tone::Success,
            ),
            card(
                "card-v-street",
                [1380.0, 540.0, 0.0],
                [450.0, 110.0],
                "Key / Value: [ street ]",
                &[("α =  2%  |  Suppressed context", Tone::Muted)],
                Tone::Plain,
            ),
            card(
                "card-v-tired",
                [1380.0, 750.0, 0.0],
                [450.0, 110.0],
                "Key / Value: [ tired ]",
                &[("α = 10%  |  Causal context", Tone::Warning)],
                Tone::Warning,
            ),
            crate::beam("beam-q-scale", "card-q", "card-scale", Tone::Accent),
            bent(
                "beam-animal",
                "card-scale",
                "card-v-animal",
                -25.0,
                Tone::Success,
            ),
            bent(
                "beam-street",
                "card-scale",
                "card-v-street",
                0.0,
                Tone::Plain,
            ),
            bent(
                "beam-tired",
                "card-scale",
                "card-v-tired",
                25.0,
                Tone::Warning,
            ),
            crate::packet("pkt-q-in", "beam-q-scale", false, "Q · Kᵀ", Tone::Accent),
            crate::packet(
                "pkt-out-animal",
                "beam-animal",
                false,
                "88% V",
                Tone::Success,
            ),
            crate::packet("pkt-out-street", "beam-street", false, "2% V", Tone::Plain),
            crate::packet("pkt-out-tired", "beam-tired", false, "10% V", Tone::Warning),
        ],
    };

    let mut s = StageActor::declare(&mut sc, "stage", &stage_plan)?;

    // Camera
    s.channel(&mut sc, "camera.z", -85.0);
    s.to(&mut sc, "camera.z", 0, 0.0, 2.2);

    // Entrances
    let q_ready = s.settle_in(&mut sc, "card-q", seconds(0.3));
    s.settle_in(&mut sc, "card-scale", seconds(0.55));
    s.settle_in(&mut sc, "card-v-animal", seconds(0.75));
    s.settle_in(&mut sc, "card-v-street", seconds(0.9));
    s.settle_in(&mut sc, "card-v-tired", seconds(1.05));

    if with_audio {
        play_bgm(&mut sc, "bgm-act4", 26.7, -14.0);
        play_voice(&mut sc, "voice-act4", "voice_act4.wav", 7.896, 0.25, 2.0);
        play_sound(&mut sc, "sfx-q", CONFIRM, q_ready, -14.0);
    }

    // Connect wires
    s.connect(&mut sc, "beam-q-scale", seconds(1.3), 0.45);
    s.connect(&mut sc, "beam-animal", seconds(1.5), 0.45);
    s.connect(&mut sc, "beam-street", seconds(1.6), 0.45);
    s.connect(&mut sc, "beam-tired", seconds(1.7), 0.45);

    // High emphasis on dominant animal beam, dim the street beam
    s.channel(&mut sc, "beam-animal.emphasis", 0.0);
    s.to(&mut sc, "beam-animal.emphasis", seconds(1.6), 1.0, 0.8);
    s.to(&mut sc, "beam-street.opacity", seconds(1.8), 0.25, 0.5);

    if with_audio {
        play_sound(&mut sc, "sfx-connect", TICK, seconds(1.3), -18.0);
    }

    // Packet from Q to scale
    let p_scale = s.send(&mut sc, "pkt-q-in", seconds(2.0), 0.5);
    s.to(&mut sc, "pkt-q-in.opacity", p_scale, 0.0, 0.15);

    if with_audio {
        play_sound(&mut sc, "sfx-send-q", SEND, seconds(2.0), -12.0);
    }

    // Packets fan out to Keys with softmax attention weights
    let p_animal = s.send(&mut sc, "pkt-out-animal", p_scale + seconds(0.2), 0.65);
    let p_street = s.send(&mut sc, "pkt-out-street", p_scale + seconds(0.35), 0.65);
    let p_tired = s.send(&mut sc, "pkt-out-tired", p_scale + seconds(0.5), 0.65);

    // Fade out packets upon arrival so their labels don't collide with the cards
    s.to(&mut sc, "pkt-out-animal.opacity", p_animal, 0.0, 0.15);
    s.to(&mut sc, "pkt-out-street.opacity", p_street, 0.0, 0.15);
    s.to(&mut sc, "pkt-out-tired.opacity", p_tired, 0.0, 0.15);

    if with_audio {
        play_sound(&mut sc, "sfx-animal-hit", IMPACT, p_animal, -14.0);
        play_sound(&mut sc, "sfx-bloom", BLOOM, p_animal + seconds(0.3), -16.0);
    }

    s.hit(&mut sc, "card-v-animal.flash", p_animal, 1.0, 0.0);
    s.hit(&mut sc, "card-v-animal.glow", p_animal, 1.0, 0.0);
    s.jolt(&mut sc, p_animal, [10.0, -3.0], 0.35);

    // Callout explaining softmax distribution
    let mut callout = CalloutActor::declare(
        &mut sc,
        "note-softmax",
        &CalloutPlan::new(
            stage_anchor("card-v-animal", "card-v-animal", CalloutSide::Top),
            vec![
                span("Softmax Distribution: ", Tone::Success),
                span(
                    "88% of incoming value flows from 'animal' based on scaled alignment.",
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
    header(&mut sc, "04", "scaled dot-product & softmax")?;
    chip(&mut sc, "ATTENTION WEIGHTS")?;
    footer(
        &mut sc,
        "footer",
        &[
            (
                "Scaled dot-products prevent saturation, while ",
                Tone::Muted,
            ),
            ("Softmax exponentiates alignment ", Tone::Accent),
            (
                "into normalized weights that selectively route relevant values.",
                Tone::Plain,
            ),
        ],
        seconds(2.6),
    )?;

    // Presentation steps
    sc.presentation_step("query", "Scaled Dot-Product Calculation", 0, seconds(2.2));
    sc.presentation_step(
        "softmax",
        "Softmax Probability Weights",
        seconds(2.2),
        seconds(5.5),
    );
    sc.presentation_step(
        "routing",
        "Selective Value Routing",
        seconds(5.5),
        seconds(9.5),
    );

    Ok(sc.finish()?)
}
