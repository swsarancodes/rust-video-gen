# From Basic NLP to Transformers: The Attention Mechanism

A cinematic, code-first 45-second motion graphics explainer created in Rust with **Psychopomp** (`wgpu` Metal graphics, `cosmic-text` layout, temporal shutter motion blur, and `ffmpeg` audio/video pipeline).

This production traces the complete conceptual and architectural revolution of modern artificial intelligence: from **classical NLP's sequential bottleneck** $\to$ **the Attention Mechanism ($Q, K, V$, scaled dot-product, softmax routing)** $\to$ **the complete Transformer Block powering today's Large Language Models (LLMs)**.

---

## 1080p 60fps Master Video (45.1s with BGM & Voiceover)

![45-Second Master Explainer Video](/Users/saran/.gemini/antigravity-ide/brain/645ce184-f128-4a06-a515-c7959d70ccfe/attention.mp4)

- **Duration**: 45.10 seconds (2,706 frames @ 60fps, 24 temporal shutter samples per frame)
- **Resolution**: Full HD 1920 × 1080
- **Audio Stack**: Ambient analog-style synthesizer soundtrack (BGM), synchronized British narrator voiceover across all 5 acts, and tactile physical sound effects (relay clicks, packet launches, dot-product impacts, and harmonic blooms).

---

## Visual Walkthrough: The 5 Evolutionary Acts

````carousel
![Act 1: The Recurrent Bottleneck (Classical NLP)](/Users/saran/.gemini/antigravity-ide/brain/645ce184-f128-4a06-a515-c7959d70ccfe/attention_act1.png)
<!-- slide -->
![Act 2: The Attention Breakthrough (Direct Parallel Routing)](/Users/saran/.gemini/antigravity-ide/brain/645ce184-f128-4a06-a515-c7959d70ccfe/attention_act2.png)
<!-- slide -->
![Act 3: The Trinity of Projections (Query, Key, Value)](/Users/saran/.gemini/antigravity-ide/brain/645ce184-f128-4a06-a515-c7959d70ccfe/attention_act3.png)
<!-- slide -->
![Act 4: Scaled Dot-Product & Softmax Routing](/Users/saran/.gemini/antigravity-ide/brain/645ce184-f128-4a06-a515-c7959d70ccfe/attention_act4.png)
<!-- slide -->
![Act 5: The Transformer Architecture (Foundation Models)](/Users/saran/.gemini/antigravity-ide/brain/645ce184-f128-4a06-a515-c7959d70ccfe/attention_act5_settled.png)
````

---

## Conceptual Architecture Breakdown

### Act 1: The Recurrent Bottleneck (`01-recurrent-bottleneck`)
- **Domain**: Classical NLP (Recurrent Neural Networks / LSTMs / GRUs).
- **The Core Problem**: In classical NLP, sentences are processed one word at a time in series. All historical context is compressed into a single, fixed-size hidden state vector $h_t \in \mathbb{R}^d$:
  $$h_t = \sigma(W_{hh} h_{t-1} + W_{xh} x_t)$$
  By the time the model reaches token 8 (`it`), information from token 1 (`animal`) has exponentially decayed or vanished. Furthermore, $O(N)$ sequential dependency prevents GPU parallel training.
- **On-Screen Choreography**:
  - Tokens `[ animal ]`, `[ street ]`, and `[ it = ? ]` connected by a sequential serial beam passing through `RNN Bottleneck: h_t`.
  - Expanding amber stress ring on the bottleneck card as recurrence packets pass through.
  - Callout: *"Information Bottleneck: Compressing the entire prefix into vector $h_t$ loses distant context."*

---

### Act 2: The Attention Breakthrough (`02-attention-breakthrough`)
- **Domain**: Parallel Direct Random-Access Routing.
- **The Core Innovation**: Attention shatters the serial chain. Instead of sequential hopping, every position establishes direct, unmediated highway connections to all other positions across the sequence in parallel:
  $$\text{Path Length} = O(1)$$
- **On-Screen Choreography**:
  - Target token `[ it ]` radiates an expanding cyan aura ring.
  - Three bent highway beams burst open to `[ animal ]`, `[ street ]`, and `[ tired ]` simultaneously.
  - Parallel probe packets race across the screen in unison with zero sequential delay.
  - Callout: *"Direct $O(1)$ Routing: Tokens examine the entire sequence in parallel without recurrent decay."*

---

### Act 3: The Trinity of Projections (`03-projections-qkv`)
- **Domain**: Learned Specialized Vector Spaces ($Q, K, V$).
- **The Core Innovation**: How does a token know what to retrieve? Each token embedding $X$ is multiplied by three independent learned weight matrices $W_Q, W_K, W_V$:
  - **Query ($Q = X \cdot W_Q$)**: *"What semantic properties am I searching for?"*
  - **Key ($K = X \cdot W_K$)**: *"What clues and properties do I advertise to other tokens?"*
  - **Value ($V = X \cdot W_V$)**: *"What is my actual semantic payload to transmit if matched?"*
- **On-Screen Choreography**:
  - Input card `Input Vector: X_it ∈ ℝᵈ` fans out into three distinct, glowing colored projection cards: Accent ($Q$), Warning ($K$), and Success ($V$).
  - Transformation packets carrying $\times W_Q$, $\times W_K$, and $\times W_V$ stream down into the projection cards with tactile landing flashes.
  - Callout: *"Linear Projections: Learned matrices decouple what a token seeks from what it transmits."*

---

### Act 4: Scaled Dot-Product & Softmax Routing (`04-scaled-dot-product-softmax`)
- **Domain**: Mathematical Compatibility & Probability Normalization.
- **The Core Mathematics**:
  1. **Directional Alignment**: Dot product $Q \cdot K^\top$ measures semantic angle.
  2. **Gradient Stabilization**: Scaled by $\frac{1}{\sqrt{d_k}}$ (e.g. $\sqrt{64} = 8$) to maintain unit variance and prevent softmax saturation.
  3. **Softmax Distribution**: Exponentiates alignment into a sharp probability distribution:
     $$\alpha = \text{Softmax}\left(\frac{Q K^\top}{\sqrt{d_k}}\right) \implies \alpha_{\text{animal}} = 88\%, \quad \alpha_{\text{tired}} = 10\%, \quad \alpha_{\text{street}} = 2\%$$
  4. **Value Aggregation**: Output is the weighted sum $Z = \sum \alpha_i V_i$.
- **On-Screen Choreography**:
  - Query card sends dot-product probe through the central $\frac{1}{\sqrt{d_k}}$ scaling card.
  - Softmax probability cards light up on the right column.
  - Heavy luminous green beam emphasizes the 88% animal connection while dimming irrelevant context.
  - Packets carry value streams into the resolved representation.

---

### Act 5: The Transformer Architecture (`05-transformer-architecture`)
- **Domain**: Modern Foundation Models (Generative AI & LLMs).
- **The Climax**: Attention is not a standalone formula; it is the core engine of the **Transformer Block**:
  1. **Multi-Head Attention (MHA)**: Runs $h$ parallel projection heads (Head 1: Coreference, Head 2: Syntactic agreement, Head 3: Causal context) to attend to information at different positions from different representation subspaces.
  2. **Residual Connections & LayerNorm**: $\text{Norm}(x + \text{MHA}(x))$ preserves gradient flow through hundreds of layers.
  3. **Position-wise Feed-Forward Network (FFN)**: 2-layer MLP with non-linear activation (SwiGLU/GeLU) that stores factual knowledge.
  4. **Deep Stack**: Stacking 32 to 128 of these blocks yields models like GPT-4, Claude, Gemini, and LLaMA.
- **On-Screen Choreography**:
  - Three distinct attention heads on the left connect into a central luminous `Transformer Block (Layer ℓ)`.
  - A golden particle orb and expanding success aura pulse as parallel head packets arrive.
  - A high-capacity output bus streams the contextualized representation $Z_\ell$ into `Deep Stack (N × Layers)`.
  - Callout: *"Universal Foundation: Stacked Transformer blocks form the backbone of all modern LLMs."*

---

## Interactive Presentation Deck

You can explore and present this deck live with keyboard navigation, step-by-step pauses, and real-time GPU motion:

```bash
cargo run --release -- plan present target/attention/deck.json --theme neutral
```

### Keyboard Shortcuts
| Key | Action |
| :--- | :--- |
| **`⌘→` / `⌘←`** | Switch slides (Acts 1 to 5) |
| **`←` / `→`** | Step forward / backward within the current act |
| **`Space` / `P`** | Pause / Resume playback |
| **`R`** | Replay current step |
| **`S` / `Shift+S`** | Cycle slow-motion speeds ($1\times$, $0.5\times$, $0.25\times$, $0.1\times$) |
| **`T` / `Shift+T`** | Switch color themes (*Neutral*, *Evergreen*, *Tokyo Night*, *Original*, *Black*) |

---

## Render Commands

```bash
# Render full 45.1s 60fps shutter-sampled video with BGM and voiceover:
cargo run --release -- plan render target/attention/attention.reel.json output/attention.mp4 --theme neutral

# Render single shutter-sampled still frame (e.g. Act 5 at t = 39.5s):
cargo run --release -- plan frame target/attention/attention.reel.json 39.5 frame.png --shutter --theme neutral
```
