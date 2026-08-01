import Munkres.Closure.Subtype
import Munkres.Mathlib.AccPt.Basic
import Munkres.Mathlib.Disjoint
import Munkres.Mathlib.Continuous
import Munkres.Subtype.Topology

import Mathlib.Data.Set.Operations

open Set Topology Filter TopologicalSpace

universe u v w

variable {α : Type u} {β : Type v} {γ : Type w}

section S₁
variable [TopologicalSpace α] [TopologicalSpace β]
  {f : α → β}

--* Theorem 18.1: Equivalence statements for continuous functions.
example : List.TFAE [ Continuous f,
  ∀ A, f '' closure A ⊆ closure (f '' A),
  ∀ B, IsClosed B → IsClosed (f ⁻¹' B),
  ∀ x, ∀ V ∈ 𝓝 (f x), ∃ U ∈ 𝓝 x, f '' U ⊆ V,
  ∀ x, ∀ V, IsOpen V → f x ∈ V → ∃ U, IsOpen U ∧ x ∈ U ∧ f '' U ⊆ V ]
  := by --
  tfae_have 1 → 2 := by
    intro hf A y ⟨x, hx, heq⟩
    subst heq
    let A' := f ⁻¹' (closure (f '' A))
    have hA' : IsClosed A' := isClosed_closure.preimage hf
    have : A ⊆ A' := image_subset_iff.mp subset_closure
    exact closure_minimal this hA' hx
  tfae_have 2 → 3 := by
    intro h B hB
    let A := f ⁻¹' B
    have hA : f '' A ⊆ B := image_preimage_subset f B
    change IsClosed A
    rw [<-closure_eq_iff_isClosed] at hB ⊢
    refine Set.Subset.antisymm ?_ subset_closure
    -- remains to show that Ā ⊆ A.
    intro x hx -- x ∈ Ā
    specialize h A (mem_image_of_mem f hx) -- f x ∈ closure (f '' A)
    replace h := closure_mono hA h
    rw [hB] at h
    exact h -- f x ∈ B ↔ x ∈ A
  tfae_have 3 → 1 := by
    intro h
    refine { isOpen_preimage := ?_ }
    intro B hB
    exact isClosed_compl_iff.mp (h Bᶜ hB.isClosed_compl)
  tfae_have 1 → 5 := by
    intro h x V hV hxV
    use f ⁻¹' V -- the pre-image is precisely the neighborhood we need.
    exact ⟨h.isOpen_preimage _ hV, hxV, image_preimage_subset f V⟩
  tfae_have 5 → 1 := by
    intro h
    refine { isOpen_preimage := ?_ }
    intro V hV
    let P := f ⁻¹' V
    let φ (x : P) : Set α := (h x.val V hV x.prop).choose
    let U := ⋃ x, φ x
    have : f ⁻¹' V = ⋃ x, φ x := by
      ext y : 1
      rw [Set.mem_iUnion]
      constructor
      · intro hy
        use ⟨y, hy⟩
        exact (h y V hV hy).choose_spec.2.1
      · intro ⟨x, hx⟩
        have hf : f '' (φ x) ⊆ V := (h x.val V hV x.prop).choose_spec.2.2
        exact hf ⟨y, hx, rfl⟩
    rw [this]
    refine isOpen_iUnion ?_
    intro x
    exact (h x.val V hV x.prop).choose_spec.1
  tfae_have 4 → 5 := by
    intro h x V hV hxV
    specialize h x V (hV.mem_nhds hxV)
    obtain ⟨U', hU', h⟩ := h
    rw [mem_nhds_iff] at hU'
    obtain ⟨U, hUU, hU, hxU⟩ := hU'
    exact ⟨U, hU, hxU, (image_mono hUU).trans h⟩
  tfae_have 5 → 4 := by
    intro h x V' hV'
    rw [mem_nhds_iff] at hV'
    obtain ⟨V, hVV, hV, hxV⟩ := hV'
    specialize h x V hV hxV
    obtain ⟨U, hU, hxU, h⟩ := h
    exact ⟨U, hU.mem_nhds hxU, h.trans hVV⟩
  tfae_finish -- ∎

end S₁

section S₂
variable [TopologicalSpace α] [TopologicalSpace β] [TopologicalSpace γ]
  {f : α → β} {g : β → γ}

--* Theorem 18.2(a): constant functions are continuous.
example {y₀ : β} : Continuous fun _ : α ↦ y₀
  := by --
  refine { isOpen_preimage := ?_ }
  intro V hV
  if h : y₀ ∈ V then
    rw [preimage_const_of_mem h]
    exact isOpen_univ
  else
    rw [preimage_const_of_notMem h]
    exact isOpen_empty -- ∎

--* Theorem 18.2(b): inclusion functions are continuous.
example {X : Set α} : Continuous fun x : X ↦ x.val
  := by --
  refine { isOpen_preimage := ?_ }
  intro V hV
  rw [isOpen_induced_iff]
  exact ⟨V, hV, rfl⟩ -- ∎

--* Theorem 18.2(c): composing two continuous functions gives a continuous function
example (hf : Continuous f) (hg : Continuous g) : Continuous (g ∘ f)
  := by --
  refine { isOpen_preimage := ?_ }
  intro V hV
  replace hg := hg.isOpen_preimage V hV
  exact hf.isOpen_preimage (g ⁻¹' V) hg -- ∎

--* Theorem 18.2(d): restricting the domain gives a continuous function.
example {s : Set α} (hf : Continuous f) : Continuous (s.imageFactorization f)
  := by --
  refine { isOpen_preimage := ?_ }
  intro W hW
  rw [isOpen_induced_iff] at hW ⊢
  obtain ⟨V, hV, heq⟩ := hW
  subst heq
  let U := f ⁻¹' V
  have hU : IsOpen U := hf.isOpen_preimage V hV
  refine ⟨U, hU, rfl⟩ -- ∎

--* Theorem 18.2(d)*: restricting the domain gives a continuous function.
example {s : Set α} (hf : Continuous f) : Continuous fun x : s ↦ f x
  := by --
  refine { isOpen_preimage := ?_ }
  intro V hV
  rw [isOpen_induced_iff]
  let U := f ⁻¹' V
  have hU := hf.isOpen_preimage V hV
  exact ⟨U, hU, rfl⟩ -- ∎

end S₂

section S₃
--* Theorem 18.3: Pasting Lemma
variable [TopologicalSpace α] [TopologicalSpace β] {A B : Set α}
  {f : A → β} {g : B → β}

private noncomputable def φ (h : A ∪ B = univ) (f : A → β) (g : B → β) : α → β := by
  intro x
  have hAB : x ∈ A ∨ x ∈ B := by change x ∈ A ∪ B; rw [h]; trivial
  if hA : x ∈ A then
    exact f ⟨x, hA⟩
  else
    have hB := hAB.resolve_left hA
    exact g ⟨x, hB⟩

example (hA : IsClosed A) (hB : IsClosed B) (hX : A ∪ B = univ)
  (hf : Continuous f) (hg : Continuous g)
  -- The functions agree on the intersection.
  (heq : ∀ x, (h : x ∈ A ∩ B) → f ⟨x, h.1⟩ = g ⟨x, h.2⟩)
  : Continuous (φ hX f g)
  := by --
  let ψ := φ hX f g
  have : Continuous ψ ↔ ∀ B, IsClosed B → IsClosed (ψ ⁻¹' B) := Continuous.tfae.out 0 2
  rw [this]
  intro C hC
  have h' (x : α) : x ∈ A ∨ x ∈ B := by change x ∈ A ∪ B; rw [hX]; exact trivial
  have : ψ ⁻¹' C = Subtype.val '' (f ⁻¹' C) ∪ Subtype.val '' (g ⁻¹' C) := by
    refine Set.Subset.antisymm ?_ ?_
    · intro x (hx : ψ x ∈ C)
      dsimp only [ψ, φ] at hx
      if hxA : x ∈ A then
        rw [dif_pos hxA] at hx
        exact Or.inl ⟨⟨x, hxA⟩, hx, rfl⟩
      else
        rw [dif_neg hxA] at hx
        have hxB := (h' x).resolve_left hxA
        exact Or.inr ⟨⟨x, hxB⟩, hx, rfl⟩
    · intro x hx
      rcases hx with hxA | hxB
      · obtain ⟨⟨y, hyA⟩, hyC, heq⟩ := hxA
        subst heq
        rw [mem_preimage]
        dsimp only [ψ, φ]
        rw [dif_pos hyA]
        exact hyC
      · obtain ⟨⟨y, hyB⟩, hyC, heq⟩ := hxB
        subst heq
        rw [mem_preimage]
        dsimp only [ψ, φ]
        if hyA : y ∈ A then
          rw [dif_pos hyA, heq y ⟨hyA, hyB⟩]
          exact hyC
        else
          rw [dif_neg hyA]
          exact hyC
  rw [this]
  refine IsClosed.union ?_ ?_
  · exact (hC.preimage hf).trans hA
  · exact (hC.preimage hg).trans hB -- ∎

end S₃
