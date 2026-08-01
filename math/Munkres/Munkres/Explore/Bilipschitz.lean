import Mathlib.Topology.MetricSpace.Defs
import Mathlib.Topology.MetricSpace.Cauchy
import Mathlib.Topology.UniformSpace.Cauchy

import Munkres.Defs

open Set Filter Topology

universe u v

variable {X : Type u} {β : Type v} [M : MetricSpace X] {x y z : X}

private def B (M : MetricSpace X) := @Metric.ball X M.toPseudoMetricSpace

private def d (x y : X) := M.dist x y
private noncomputable def f (t : ℝ) : ℝ := t / (1 + t)
private noncomputable def ρ (x y : X) : ℝ := f (d x y)

private lemma hd₁ {x y : X} : 0 < 1 + dist x y := add_pos_of_pos_of_nonneg zero_lt_one dist_nonneg

private lemma hf : MonotoneOn f (Ici 0)
  := by --
  intro a (ha : 0 ≤ a) b (hb : 0 ≤ b) hab
  refine (div_le_div_iff₀ ?_ ?_).mpr ?_
  · exact add_pos_of_pos_of_nonneg zero_lt_one ha
  · exact add_pos_of_pos_of_nonneg zero_lt_one hb
  · simp only [left_distrib, mul_one]
    refine add_le_add hab ?_
    rw [mul_comm] -- ∎

private lemma hf₁ {t : ℝ} (ht : 0 ≤ t) : f t ≤ 1
  := by --
  dsimp only [f]
  have : 0 < 1 + t := by
    refine zero_lt_one.trans_le ?_
    exact (le_add_iff_nonneg_right 1).mpr ht
  rw [div_le_one this, le_add_iff_nonneg_left]
  exact zero_le_one -- ∎

-- ρ = d / (1 + d) is a metric.
@[reducible]
private noncomputable def M' (X : Type u) [MetricSpace X] : MetricSpace X
  := by --
  exact {
    dist := ρ
    dist_comm x y := by dsimp only [ρ, d]; rw [dist_comm]
    dist_self x := by dsimp only [ρ, d, f]; rw [dist_self, zero_div]
    eq_of_dist_eq_zero {x y} h₀ := by
      dsimp only [ρ, f] at h₀
      rw [div_eq_zero_iff] at h₀
      rcases h₀ with h₀ | h₀
      · exact eq_of_dist_eq_zero h₀
      · exact False.elim (hd₁.ne' h₀)
    dist_triangle x y z := by
      have : f (d x z) ≤ f (d x y + d y z) := by
        refine hf dist_nonneg ?_ ?_
        · exact add_nonneg dist_nonneg dist_nonneg
        · exact dist_triangle _ _ _
      refine this.trans ?_
      dsimp only [f]
      rw [add_div] -- the crux
      refine add_le_add ?_ ?_
      · refine div_le_div_of_nonneg_left ?_ hd₁ ?_
        · exact dist_nonneg
        · refine add_le_add_right ?_ 1
          exact (le_add_iff_nonneg_right _).mpr dist_nonneg
      · refine div_le_div_of_nonneg_left ?_ hd₁ ?_
        · exact dist_nonneg
        · refine add_le_add_right ?_ 1
          exact (le_add_iff_nonneg_left _).mpr dist_nonneg
  } -- ∎
@[reducible]
private noncomputable def P := M.toPseudoMetricSpace
@[reducible]
private noncomputable def P' (X : Type u) [MetricSpace X] := (M' X).toPseudoMetricSpace
@[reducible]
private noncomputable def P₁ := (M' X).toPseudoMetricSpace
@[reducible]
private noncomputable def U := M.toPseudoMetricSpace.toUniformSpace
@[reducible]
private noncomputable def U' (X : Type u) [MetricSpace X] :=
  (M' X).toPseudoMetricSpace.toUniformSpace
@[reducible]
private noncomputable def U₁ := (M' X).toPseudoMetricSpace.toUniformSpace
@[reducible]
private noncomputable def T := M.toPseudoMetricSpace.toUniformSpace.toTopologicalSpace
@[reducible]
private noncomputable def T' (X : Type u) [MetricSpace X] :=
  (M' X).toPseudoMetricSpace.toUniformSpace.toTopologicalSpace
@[reducible]
private noncomputable def T₁ := (M' X).toPseudoMetricSpace.toUniformSpace.toTopologicalSpace

private lemma ρ_eq : ρ = (M' X).dist := rfl

private lemma ρ_le_d : ρ x y ≤ d x y
  := by --
  dsimp only [ρ, f]
  refine div_le_self dist_nonneg ?_
  exact (le_add_iff_nonneg_right 1).mpr dist_nonneg -- ∎

private lemma d_le_ρ (hle : d x y ≤ 1) : d x y ≤ 2 * ρ x y
  := by --
  have : d x y / (1 + 1) ≤ d x y / (1 + d x y) := by
    refine div_le_div_of_nonneg_left dist_nonneg ?_ ?_
    · exact add_pos_of_pos_of_nonneg zero_lt_one dist_nonneg
    · exact add_le_add_right hle 1
  refine (mul_le_mul_of_nonneg_left this zero_le_two).trans' ?_
  rw [one_add_one_eq_two, mul_div_cancel₀ _ two_ne_zero] -- ∎

private lemma d_le_ρ' (hle : ρ x y ≤ 1 / 2) : d x y ≤ 2 * ρ x y
  := by --
  have h : 2 * ρ x y ≤ 1 := by
    rw [<-mul_div_cancel₀ (1 : ℝ) two_ne_zero]
    exact mul_le_mul_of_nonneg_left hle zero_le_two
  dsimp only [ρ, f] at h
  rw [mul_div] at h
  have h : 2 * d x y ≤ 1 * (1 + d x y) := (div_le_iff₀ hd₁).mp h
  rw [one_mul, two_mul] at h
  rw [add_le_add_iff_right (d x y)] at h
  exact d_le_ρ h -- ∎

private theorem t₁ {ε : ℝ} (hε : ε ≤ 1) : B (M' X) x (ε / 2) ⊆ B M x ε
  := by --
  intro y (hy : ρ y x < ε / 2)
  -- change d y x < ε
  have hy' : 2 * ρ y x < ε := by
    rw [<-mul_div_cancel₀ ε two_ne_zero]
    exact mul_lt_mul_of_pos_left hy zero_lt_two
  refine (d_le_ρ' ?_).trans_lt hy'
  refine hy.le.trans ?_
  exact div_le_div_of_nonneg_right hε zero_le_two -- ∎

private theorem t₂ {ε : ℝ} : B M x ε ⊆ B (M' X) x ε
  := by --
  intro y (hy : d y x < ε)
  -- change ρ y x < ε
  exact ρ_le_d.trans_lt hy -- ∎

private def MetricSpace.CauchySeq (M : MetricSpace β) (f : ℕ → β) := _root_.CauchySeq f

private theorem cauchy_iff₀ (f : ℕ → X) : M.CauchySeq f ↔ (M' X).CauchySeq f
  := by --
  simp only [MetricSpace.CauchySeq]
  rw [Metric.cauchySeq_iff']
  rw [@Metric.cauchySeq_iff' X ℕ (P' X)]
  change (∀ ε > 0, ∃ N, ∀ n ≥ N, d (f n) (f N) < ε) ↔ ∀ ε > 0, ∃ N, ∀ n ≥ N, ρ (f n) (f N) < ε
  constructor
  · intro h ε hε
    specialize h ε hε
    obtain ⟨N, h⟩ := h
    use N
    intro n hn
    specialize h n hn
    exact ρ_le_d.trans_lt h
  · intro h ε' hε'
    let ε := min (1 / 2) (ε' / 3)
    have hε : 0 < ε := lt_min one_half_pos (div_pos hε' zero_lt_three)
    specialize h ε hε
    obtain ⟨N, h⟩ := h
    use N
    intro n hn
    specialize h n hn
    have hρ₁ : ρ (f n) (f N) ≤ 1 / 2 := h.le.trans (min_le_left _ _)
    refine (d_le_ρ' hρ₁).trans_lt ?_
    have : 2 * ρ (f n) (f N) < 2 * ε := mul_lt_mul_of_pos_left h zero_lt_two
    refine this.trans ?_
    refine (lt_div_iff₀' zero_lt_two).mp ?_
    dsimp only [ε]
    refine (min_le_right _ _).trans_lt ?_
    refine div_lt_div_of_pos_left hε' zero_lt_two ?_
    norm_num -- ∎

private theorem t_eq : T = T' X
  := by --
  refine TopologicalSpace.ext ?_
  ext U : 2
  rw [@Metric.isOpen_iff X P, @Metric.isOpen_iff X (P' X)]
  constructor
  · intro h x hxU
    specialize h x hxU
    obtain ⟨ε', hε', h⟩ := h
    let ε := min (1 / 2) (ε' / 3)
    have hε : 0 < ε := lt_min one_half_pos (div_pos hε' zero_lt_three)
    refine ⟨ε, hε, ?_⟩
    refine h.trans' ?_
    rintro y (hy : ρ y x < ε)
    change d y x < ε'
    have : ρ y x ≤ 1 / 2 := hy.le.trans (min_le_left _ _)
    refine (d_le_ρ' this).trans_lt ?_
    have := mul_lt_mul_of_pos_left hy zero_lt_two
    refine this.trans ?_
    refine (lt_div_iff₀' zero_lt_two).mp ?_
    dsimp only [ε]
    refine (min_le_right _ _).trans_lt ?_
    refine div_lt_div_of_pos_left hε' zero_lt_two ?_
    norm_num
  · intro h x hxU
    specialize h x hxU
    obtain ⟨ε, hε, h⟩ := h
    refine ⟨ε, hε, ?_⟩
    refine h.trans' ?_
    intro y hy
    exact ρ_le_d.trans_lt hy -- ∎

private lemma c₁ : @CompleteSpace X U → @CompleteSpace X (U' X)
  := by --
  intro h
  rw [@Munkres.Metric.CompleteSpace_iff X M] at h
  rw [@Munkres.Metric.CompleteSpace_iff X (M' X)]
  intro f (hf : (M' X).CauchySeq f)
  rw [<-cauchy_iff₀ f] at hf
  specialize h f hf
  obtain ⟨x, h⟩ := h
  refine ⟨x, ?_⟩
  rw [Metric.tendsto_atTop] at h
  rw [@Metric.tendsto_atTop X ℕ (P' X)]
  intro ε hε
  specialize h ε hε
  obtain ⟨N, h⟩ := h
  use N
  intro n hn
  specialize h n hn
  change ρ (f n) x < ε
  refine ρ_le_d.trans_lt ?_
  exact h -- ∎

private lemma c₂ : @CompleteSpace X (U' X) → @CompleteSpace X U
  := by --
  intro h
  rw [@Munkres.Metric.CompleteSpace_iff X (M' X)] at h
  rw [@Munkres.Metric.CompleteSpace_iff X M]
  intro f (hf : M.CauchySeq f)
  rw [cauchy_iff₀ f] at hf
  specialize h f hf
  obtain ⟨x, h⟩ := h
  refine ⟨x, ?_⟩
  rw [Metric.tendsto_atTop]
  rw [@Metric.tendsto_atTop X ℕ (P' X)] at h
  change ∀ ε > 0, ∃ N, ∀ n ≥ N, ρ (f n) x < ε at h
  intro ε' hε'
  let ε := min (1 / 2) (ε' / 2)
  have hε : 0 < ε := lt_min one_half_pos (half_pos hε')
  specialize h ε hε
  obtain ⟨N, h⟩ := h
  use N
  intro n hn
  specialize h n hn
  change d (f n) x < ε'
  have hdρ := d_le_ρ' <| h.le.trans (min_le_left _ _)
  refine hdρ.trans_lt ?_
  have : 2 * ρ (f n) x < 2 * ε := mul_lt_mul_of_pos_left h zero_lt_two
  refine this.trans_le ?_
  refine (le_mul_inv_iff₀' zero_lt_two).mp ?_
  exact min_le_right _ _ -- ∎

example : @CompleteSpace X U ↔ @CompleteSpace X (U' X) := ⟨c₁, c₂⟩

variable {Λ : Type v} {x y : Λ → X}

/-- ρ-bar. -/
private noncomputable def b (x y : Λ → X) : ℝ := sSup { ρ (x α) (y α) | α : Λ }

@[reducible]
private def subsingleton_ms : Subsingleton Λ → MetricSpace Λ
  := by --
  intro h
  exact {
    dist := 0
    dist_self x := rfl
    dist_comm x y := rfl
    dist_triangle x y z := by
      simp only [Pi.zero_apply, add_zero]
      exact le_rfl
    eq_of_dist_eq_zero {x y} _ := h.allEq x y
  } -- ∎


private theorem hρ₁ : 1 ∈ upperBounds { ρ (x α) (y α) | α : Λ }
  := by --
  intro d ⟨α, heq⟩
  subst heq
  exact hf₁ dist_nonneg -- ∎

private theorem ρ_bdd_above : BddAbove { ρ (x α) (y α) | α : Λ }
  := by --
  exact ⟨1, hρ₁⟩ -- ∎

theorem sSup_le_add {A B : Set ℝ}
  (hA₀ : A.Nonempty) (hB₀ : B.Nonempty)
  (hA : BddAbove A) (hB : BddAbove B)
  : sSup { a + b | (a ∈ A) (b ∈ B) } ≤ sSup A + sSup B
  := by --
  refine csSup_le ?_ ?_
  · obtain ⟨a, ha⟩ := hA₀
    obtain ⟨b, hb⟩ := hB₀
    exact ⟨a + b, a, ha, b, hb, rfl⟩
  · intro v ⟨a, ha, b, hb, heq⟩
    subst heq
    refine add_le_add ?_ ?_
    · exact le_csSup hA ha
    · exact le_csSup hB hb -- ∎

@[reducible]
private noncomputable def M₂ (Λ : Type v) (X : Type u) [MetricSpace X] : MetricSpace (Λ → X)
  := by --
  exact {
    dist := b
    dist_comm x y := by
      dsimp only [b]
      simp only [ρ_eq, (M' X).dist_comm]
    dist_self x := by
      dsimp only [b]
      simp only [ρ_eq, (M' X).dist_self]
      if h₀ : IsEmpty Λ then
        simp only [IsEmpty.exists_iff, setOf_false, Real.sSup_empty]
      else
      replace h₀ : Nonempty Λ := not_isEmpty_iff.mp h₀
      refine le_antisymm ?_ ?_
      · refine csSup_le ?_ ?_
        · exact ⟨0, h₀.some, rfl⟩
        · intro b ⟨_, hb⟩
          exact hb.ge
      · refine le_csSup ?_ ?_
        · refine ⟨1, ?_⟩
          intro x ⟨_, hx⟩
          subst hx
          exact zero_le_one
        · exact ⟨h₀.some, rfl⟩
    eq_of_dist_eq_zero {x y} heq := by
      dsimp only [b] at heq
      funext α
      refine (M' X).eq_of_dist_eq_zero ?_
      refine le_antisymm ?_ (@dist_nonneg _ P₁ _ _)
      rw [<-heq]
      exact le_csSup ⟨1, hρ₁⟩ ⟨α, rfl⟩
    dist_triangle x y z := by
      if h₀ : IsEmpty Λ then
        dsimp only [b]
        simp only [IsEmpty.exists_iff, setOf_false, Real.sSup_empty, add_zero, le_refl]
      else
      replace h₀ : Nonempty Λ := not_isEmpty_iff.mp h₀
      let ζ (x y : Λ → X) := { ρ (x α) (y α) | α }
      change sSup (ζ x z) ≤ sSup (ζ x y) + sSup (ζ y z)
      let α := h₀.some
      have h₀ (x y : Λ → X) : (ζ x y).Nonempty := ⟨ρ (x α) (y α), α, rfl⟩
      have := sSup_le_add (A := ζ x y) (B := ζ y z) (h₀ x y) (h₀ y z) ⟨1, hρ₁⟩ ⟨1, hρ₁⟩
      refine this.trans' ?_
      refine csSup_le (h₀ x z) ?_
      intro v ⟨α, heq⟩
      subst heq
      have : ρ (x α) (z α) ≤ ρ (x α) (y α) + ρ (y α) (z α) := @dist_triangle _ P₁ (x α) (y α) (z α)
      refine this.trans ?_
      refine le_csSup ?_ ?_
      · refine ⟨2, ?_⟩
        intro v ⟨a, ha, b, hb, heq⟩
        subst heq
        rw [<-one_add_one_eq_two]
        exact add_le_add (hρ₁ ha) (hρ₁ hb)
      · refine ⟨ρ (x α) (y α), ?_, ρ (y α) (z α), ?_, rfl⟩
        · exact ⟨α, rfl⟩
        · exact ⟨α, rfl⟩
  } -- ∎
@[reducible]
private noncomputable def P₂ := (M₂ Λ X).toPseudoMetricSpace
@[reducible]
private noncomputable def U₂ := (M₂ Λ X).toPseudoMetricSpace.toUniformSpace
@[reducible]
private noncomputable def T₂ := (M₂ Λ X).toPseudoMetricSpace.toUniformSpace.toTopologicalSpace

private lemma b_eq : b = (M₂ Λ X).dist := rfl

private lemma c₃ : @CompleteSpace X (U' X) → @CompleteSpace (Λ → X) U₂
  := by --
  intro h
  -- Handle the trivial case so that we have `Nonempty Λ` for csSup_le later
  if h₀ : IsEmpty Λ then
    have h₁ : Subsingleton (Λ → X) := Unique.instSubsingleton
    have h₀ : Nonempty (Λ → X) := instNonemptyOfInhabited
    rw [@Munkres.Metric.CompleteSpace_iff _ (M₂ Λ X)]
    intro f hf
    use h₀.some
    rw [nhds_discrete, tendsto_pure, eventually_atTop]
    use 0
    intro _ _
    exact h₁.allEq _ _
  else
  have h₀ : Nonempty Λ := not_isEmpty_iff.mp h₀
  rw [@Munkres.Metric.CompleteSpace_iff X (M' X)] at h
  rw [@Munkres.Metric.CompleteSpace_iff _ (M₂ Λ X)]
  intro f hf
  -- Fixing α, each fₙ(α) is a Cauchy Sequence.
  have hc (α : Λ) : @CauchySeq _ _ U₁ _ (fun n ↦ f n α) := by
    rw [@Metric.cauchySeq_iff' _ _ P₁]
    rw [@Metric.cauchySeq_iff'] at hf
    intro ε hε
    specialize hf ε hε
    obtain ⟨N, hf⟩ := hf
    use N
    intro n hn
    specialize hf n hn
    exact (le_csSup ⟨1, hρ₁⟩ ⟨α, rfl⟩).trans_lt hf
  -- So for each α, fₙ(α) converges to a point in X. Consolidate all these
  -- points into a function. This will be the limit.
  let y (α : Λ) : X := (h (f · α) (hc α)).choose
  use y
  rw [@Metric.tendsto_atTop]
  rw [@Metric.cauchySeq_iff] at hf
  -- We have to choose the first N. At this point we obly have that fₙ is Cauchy.
  -- Use that first.
  intro E hE
  obtain ⟨ε, hε, hεE⟩ := exists_between hE
  specialize hf (ε / 2) (half_pos hε)
  obtain ⟨N, hf : ∀ m ≥ N, ∀ n ≥ N, b (f m) (f n) < ε / 2⟩ := hf
  use N
  intro n hn -- Fix the `n` first. We'll use the `m` next, right away.

  -- Use the supremum-ness of b (ρ-bar) to trickle down to a "∀ α" situation.
  have h₂ (α : Λ) : ∀ m ≥ N, ρ (f n α) (f m α) < ε / 2 := by
    intro m hm
    specialize hf n hn m hm
    exact (le_csSup ⟨1, hρ₁⟩ ⟨α, rfl⟩).trans_lt hf
  -- so now, we have that for all α ∈ Λ, ρ(fₙ(α),fₘ(α)) < ε / 2, and we can
  -- obtain ρ(fₘ(α),f(α)) < ε / 2 from the fact that fₘ(α) → f(α) as m → ∞.
  refine hεE.trans_le' ?_
  refine csSup_le ?_ ?_
  · let α := h₀.some -- here we use the Nonempty Λ we secured way early on.
    exact ⟨ρ (f n α) (y α), α, rfl⟩ -- csSup_le is happy.
  · intro v ⟨α, heq⟩
    subst heq
    have htt : Tendsto (f · α) atTop (@nhds _ T₁ (y α)) := (h (f · α) (hc α)).choose_spec
    rw [@Metric.tendsto_atTop _ _ P₁] at htt
    specialize h₂ α
    specialize htt (ε / 2) (half_pos hε)
    obtain ⟨N', htt : ∀ n ≥ N', ρ (f n α) (y α) < ε / 2⟩ := htt
    let M := max N N'
    specialize h₂ M (le_max_left _ _)
    specialize htt M (le_max_right _ _)
    rw [ρ_eq]
    refine (@dist_triangle _ P₁ (f n α) (f M α) (y α)).trans ?_
    rw [<-add_halves ε]
    exact add_le_add h₂.le htt.le -- ∎
