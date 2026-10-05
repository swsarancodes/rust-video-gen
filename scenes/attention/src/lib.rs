//! Psychopomp Attention Explainer
//!
//! A cinematic motion-graphics explainer in Rust on how the Attention
//! Mechanism works in modern Transformers and Large Language Models.
//!
//! Visualized in 5 acts:
//! 1. The Context Dilemma: Why static embeddings fail on reference and polysemy.
//! 2. The Trinity of Projections: Deriving Query (Q), Key (K), and Value (V).
//! 3. Scaled Dot-Product: Computing semantic alignment (Q · Kᵀ / √d_k).
//! 4. Softmax Normalization: Transforming scores into attention weights and routing Value.
//! 5. Contextualized Synthesis: Integrating multi-head attention into a contextual embedding.

pub mod compatibility;
pub mod dilemma;
pub mod projections;
pub mod softmax;
pub mod synthesis;

use anyhow::Result;
use std::path::PathBuf;

use psychopomp::{
    author::{PlanBuilder, seconds},
    callout::{CalloutAnchorPlan, CalloutSide},
    caption::{CaptionActor, CaptionAlign, CaptionPlan, CaptionSpanPlan},
    effects::spinner::Mark,
    plan::{
        DeckPlan, MediaKindPlan, MediaPlan, MediaRolePlan, ReelPlan, ReelSegmentPlan,
        ReelTransitionStyle, SlidePlan,
    },
    stage::{StageActor, StageElement, StagePost, StatusText},
    tone::Tone,
};

pub const TRANSITION_SECONDS: f64 = 0.6;
pub const LEFT: f32 = 140.0;
pub const RIGHT: f32 = 1780.0;
pub const HEADER_Y: f32 = 96.0;
pub const FOOTER_Y: f32 = 1004.0;
pub const SCENE_DURATION_SECONDS: f64 = 9.5;

/// Builds the complete 5-act explainer as a seamless Reel with transitions.
pub fn build_reel(with_audio: bool) -> Result<ReelPlan> {
    let segments = vec![
        dilemma::scene(with_audio)?,
        projections::scene(with_audio)?,
        compatibility::scene(with_audio)?,
        softmax::scene(with_audio)?,
        synthesis::scene(with_audio)?,
    ];

    let reel = ReelPlan {
        version: ReelPlan::VERSION,
        id: "attention-explainer".to_owned(),
        segments: segments
            .into_iter()
            .enumerate()
            .map(|(index, plan)| ReelSegmentPlan {
                transition_nanos: if index == 0 {
                    0
                } else {
                    seconds(TRANSITION_SECONDS)
                },
                transition_style: ReelTransitionStyle::Dip,
                transition_focus: None,
                transition_wipe: None,
                plan,
            })
            .collect(),
    };
    reel.validate()?;
    Ok(reel)
}

/// Builds an interactive Presentation Deck for `psychopomp plan present`.
pub fn build_deck() -> Result<DeckPlan> {
    let slides = vec![
        SlidePlan {
            title: "01  The Recurrent Bottleneck (Classical NLP)".to_owned(),
            plan: dilemma::scene(false)?,
        },
        SlidePlan {
            title: "02  The Attention Breakthrough (Parallel Routing)".to_owned(),
            plan: projections::scene(false)?,
        },
        SlidePlan {
            title: "03  The Trinity: Query, Key, Value".to_owned(),
            plan: compatibility::scene(false)?,
        },
        SlidePlan {
            title: "04  Scaled Dot-Product & Softmax Routing".to_owned(),
            plan: softmax::scene(false)?,
        },
        SlidePlan {
            title: "05  The Transformer Architecture (Foundation Models)".to_owned(),
            plan: synthesis::scene(false)?,
        },
    ];

    let deck = DeckPlan {
        version: 1,
        id: "attention-deck".to_owned(),
        slides,
    };
    deck.validate()?;
    Ok(deck)
}

// ---------------------------------------------------------------------------
// Shared visual & audio helpers
// ---------------------------------------------------------------------------

pub fn span(text: &str, tone: Tone) -> CaptionSpanPlan {
    CaptionSpanPlan::new(text, tone)
}

pub fn spans(parts: &[(&str, Tone)]) -> Vec<CaptionSpanPlan> {
    parts.iter().map(|(text, tone)| span(text, *tone)).collect()
}

pub fn stage_anchor(id: &str, element: &str, edge: CalloutSide) -> CalloutAnchorPlan {
    CalloutAnchorPlan::Stage {
        id: id.into(),
        element: element.into(),
        edge,
        side: None,
    }
}

pub fn header(sc: &mut PlanBuilder, number: &str, title: &str) -> Result<()> {
    let plan = CaptionPlan::line(
        [LEFT, HEADER_Y],
        30.0,
        vec![
            span("ATTENTION  ", Tone::Accent),
            span(number, Tone::Plain),
            span("  ", Tone::Muted),
            span(title, Tone::Plain),
        ],
    );
    let mut caption = CaptionActor::declare(sc, "header", &plan)?;
    caption.type_in(sc, seconds(0.2), 48.0, 0.5);
    Ok(())
}

pub fn chip(sc: &mut PlanBuilder, text: &str) -> Result<CaptionActor> {
    let plan = CaptionPlan::line(
        [RIGHT, HEADER_Y],
        22.0,
        vec![span("TRANSFORMER ", Tone::Muted), span(text, Tone::Accent)],
    )
    .aligned(CaptionAlign::Right)
    .chip();
    let mut actor = CaptionActor::declare(sc, "chip", &plan)?;
    actor.show(sc, seconds(0.4));
    Ok(actor)
}

pub fn footer(
    sc: &mut PlanBuilder,
    id: &str,
    parts: &[(&str, Tone)],
    at: u64,
) -> Result<CaptionActor> {
    let mut caption = CaptionActor::declare(
        sc,
        id,
        &CaptionPlan::line([LEFT, FOOTER_Y], 26.0, spans(parts)),
    )?;
    caption.type_in(sc, at, 42.0, 0.7);
    Ok(caption)
}

pub fn post() -> StagePost {
    StagePost {
        bloom: 0.32,
        grain: 0.015,
        vignette: 0.28,
        backdrop: 0.16,
    }
}

pub fn card(
    id: &str,
    at: [f32; 3],
    size: [f32; 2],
    title: &str,
    status: &[(&str, Tone)],
    tone: Tone,
) -> StageElement {
    StageElement::Card {
        id: id.into(),
        at,
        size,
        title: title.into(),
        status: status
            .iter()
            .map(|(text, tone)| StatusText {
                text: (*text).to_owned(),
                tone: *tone,
            })
            .collect(),
        tone,
        mark: Mark::Check,
    }
}

pub fn orb(id: &str, at: [f32; 3], radius: f32, points: u32, tone: Tone) -> StageElement {
    StageElement::Orb {
        id: id.into(),
        at,
        radius,
        points,
        tone,
    }
}

pub fn beam(id: &str, from: &str, to: &str, tone: Tone) -> StageElement {
    bent(id, from, to, 0.0, tone)
}

pub fn bent(id: &str, from: &str, to: &str, bend: f32, tone: Tone) -> StageElement {
    StageElement::Beam {
        id: id.into(),
        from: from.into(),
        to: to.into(),
        bend,
        tone,
    }
}

pub fn packet(id: &str, beam: &str, reverse: bool, label: &str, tone: Tone) -> StageElement {
    StageElement::Packet {
        id: id.into(),
        beam: beam.into(),
        reverse,
        label: label.into(),
        tone,
    }
}

pub fn ring(id: &str, at: [f32; 3], radius: f32, thickness: f32, tone: Tone) -> StageElement {
    StageElement::Ring {
        id: id.into(),
        at,
        radius,
        thickness,
        tone,
    }
}

pub fn label(
    id: &str,
    at: [f32; 3],
    size: f32,
    align: CaptionAlign,
    parts: &[(&str, Tone)],
) -> StageElement {
    StageElement::Label {
        id: id.into(),
        at,
        size,
        align,
        spans: spans(parts),
    }
}

pub fn orb_in(s: &mut StageActor, sc: &mut PlanBuilder, id: &str, at: u64) {
    for (property, initial) in [("scale", 0.5), ("blur", 12.0), ("rotation", -2.0)] {
        s.channel(sc, &format!("{id}.{property}"), initial);
    }
    s.bounce(sc, &format!("{id}.scale"), at, 1.0, 0.85, 0.2);
    s.to(sc, &format!("{id}.blur"), at, 0.0, 0.6);
    s.ease(
        sc,
        &format!("{id}.rotation"),
        at,
        0.0,
        1.2,
        psychopomp::math::easing::Ease::CubicOut,
    );
    s.to(sc, &format!("{id}.opacity"), at, 1.0, 0.5);
}

// ---------------------------------------------------------------------------
// Sound effects helpers
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
pub struct Sfx(&'static str, f64);

pub const TICK: Sfx = Sfx("visual-effects/task-running.wav", 0.13);
pub const SEND: Sfx = Sfx("opencode-hot-reload/save.wav", 0.15);
pub const SUCCESS: Sfx = Sfx("visual-effects/task-success.wav", 0.41);
pub const IMPACT: Sfx = Sfx("opencode-hot-reload/impact.wav", 0.51);
pub const BLOOM: Sfx = Sfx("effect-shows-errors/prismatic-bloom.wav", 0.785);
pub const CONFIRM: Sfx = Sfx("opencode-hot-reload/confirm.wav", 0.33);

pub fn play_sound(sc: &mut PlanBuilder, id: &str, sfx: Sfx, at: u64, gain_db: f32) {
    let Sfx(file, length_sec) = sfx;
    let length = seconds(length_sec);
    let asset_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .join(file);
    sc.media(MediaPlan {
        id: id.to_owned(),
        path: asset_path,
        kind: MediaKindPlan::Audio,
        role: MediaRolePlan::Layer,
        source_start_nanos: 0,
        source_end_nanos: length,
        timeline_start_nanos: at,
        timeline_end_nanos: at + length,
        gain_db,
    });
}

pub fn play_bgm(sc: &mut PlanBuilder, id: &str, span_start_sec: f64, gain_db: f32) {
    let source_start = seconds(span_start_sec);
    let source_end = seconds(span_start_sec + SCENE_DURATION_SECONDS);
    let asset_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/attention/bgm.wav");
    sc.media(MediaPlan {
        id: id.to_owned(),
        path: asset_path,
        kind: MediaKindPlan::Audio,
        role: MediaRolePlan::Layer,
        source_start_nanos: source_start,
        source_end_nanos: source_end,
        timeline_start_nanos: 0,
        timeline_end_nanos: seconds(SCENE_DURATION_SECONDS),
        gain_db,
    });
}

pub fn play_voice(
    sc: &mut PlanBuilder,
    id: &str,
    wav_filename: &str,
    length_sec: f64,
    at_sec: f64,
    gain_db: f32,
) {
    let length = seconds(length_sec);
    let at = seconds(at_sec);
    let asset_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/attention")
        .join(wav_filename);
    sc.media(MediaPlan {
        id: id.to_owned(),
        path: asset_path,
        kind: MediaKindPlan::Audio,
        role: MediaRolePlan::Script,
        source_start_nanos: 0,
        source_end_nanos: length,
        timeline_start_nanos: at,
        timeline_end_nanos: at + length,
        gain_db,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_reel_and_deck_are_valid() {
        let reel = build_reel(false).unwrap();
        reel.validate().unwrap();

        let deck = build_deck().unwrap();
        deck.validate().unwrap();
    }
}
