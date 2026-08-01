import Mathlib.Tactic.TFAE
import Mathlib.Topology.Continuous

open Set Topology

universe u v

variable {α : Type u} {β : Type v}
  [TopologicalSpace α] [TopologicalSpace β]
  {f : α → β}

theorem Continuous.tfae : List.TFAE [ Continuous f,
  ∀ A, f '' closure A ⊆ closure (f '' A),
  ∀ B, IsClosed B → IsClosed (f ⁻¹' B),
  ∀ x, ∀ V ∈ 𝓝 (f x), ∃ U ∈ 𝓝 x, f '' U ⊆ V,
  ∀ x, ∀ V, IsOpen V → f x ∈ V → ∃ U, IsOpen U ∧ x ∈ U ∧ f '' U ⊆ V ]
  := by --
  tfae_have 1 → 2 := fun hf A ↦ image_closure_subset_closure_image hf
  tfae_have 1 ↔ 3 := continuous_iff_isClosed
  tfae_have 1 → 2 := by --
    intro hf A y ⟨x, hx, heq⟩
    subst heq
    let A' := f ⁻¹' (closure (f '' A))
    have hA' : IsClosed A' := isClosed_closure.preimage hf
    have : A ⊆ A' := image_subset_iff.mp subset_closure
    exact closure_minimal this hA' hx -- ∎
  tfae_have 2 → 3 := by --
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
    exact isClosed_compl_iff.mp (h Bᶜ hB.isClosed_compl) -- ∎
  tfae_have 1 → 5 := by --
    intro h x V hV hxV
    use f ⁻¹' V -- the pre-image is precisely the neighborhood we need.
    exact ⟨h.isOpen_preimage _ hV, hxV, image_preimage_subset f V⟩ -- ∎
  tfae_have 5 → 1 := by --
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
    exact (h x.val V hV x.prop).choose_spec.1 -- ∎
  tfae_have 4 → 5 := by --
    intro h x V hV hxV
    specialize h x V (hV.mem_nhds hxV)
    obtain ⟨U', hU', h⟩ := h
    rw [mem_nhds_iff] at hU'
    obtain ⟨U, hUU, hU, hxU⟩ := hU'
    exact ⟨U, hU, hxU, (image_mono hUU).trans h⟩ -- ∎
  tfae_have 5 → 4 := by --
    intro h x V' hV'
    rw [mem_nhds_iff] at hV'
    obtain ⟨V, hVV, hV, hxV⟩ := hV'
    specialize h x V hV hxV
    obtain ⟨U, hU, hxU, h⟩ := h
    exact ⟨U, hU.mem_nhds hxU, h.trans hVV⟩ -- ∎
  tfae_finish -- ∎
