import Mathlib.Topology.Separation.Basic
import Munkres.Defs.Basic

open Filter Set Munkres
open scoped Topology

universe u v

variable {α : Type u} [TopologicalSpace α] {s : Set α} {x : α}

/-- Munkres defines that x is a limit point of A if every open U ⊆ X containing
x intersects with A \ {x}. This is equivalent to Mathlib's `AccPt x (𝓟 A)`. -/
protected theorem AccPt.iff
  : AccPt x (𝓟 s) ↔ ∀ u, IsOpen u → x ∈ u → (u ∩ (s \ {x})).Nonempty
  := by --
  rw [accPt_principal_iff_clusterPt, clusterPt_principal_iff]
  refine ⟨(· · <| ·.mem_nhds ·), ?_⟩
  intro h v hv
  rw [mem_nhds_iff] at hv
  obtain ⟨u, huv, hu, hxu⟩ := hv
  exact (h u hu hxu).mono (inter_subset_inter_left _ huv) -- ∎

theorem mem_closure_iff_accPt {x : α} : x ∈ closure s ↔ x ∈ s ∨ AccPt x (𝓟 s)
  := by --
  rw [mem_closure_iff_clusterPt]
  exact clusterPt_principal -- ∎

--* closure A = A ∪ A'
theorem AccPt.union_eq_closure : s ∪ { x | AccPt x (𝓟 s) } = closure s
  := by --
  refine Set.ext fun x ↦ ?_
  simp only [mem_closure_iff_accPt]
  exact Eq.to_iff rfl -- ∎

theorem AccPt.mem_closure (h : AccPt x (𝓟 s)) : x ∈ closure s
  := by --
  exact mem_closure_iff_clusterPt.mpr h.clusterPt -- ∎

-- Alternative proof.
example (h : AccPt x (𝓟 s)) : x ∈ closure s
  := by --
  rw [<-AccPt.union_eq_closure]
  exact mem_union_right s h -- ∎

theorem AccPt.of_tendsto {β : Type v} [Nonempty β] [SemilatticeSup β]
  {f : β → α} (hs : ∀ᶠ n in atTop, f n ∈ s) (h : Tendsto f atTop (𝓝[≠] x))
  : AccPt x (𝓟 s)
  := by --
  rw [AccPt.iff]
  intro U hU hxU
  rw [tendsto_nhdsWithin_iff] at h
  obtain ⟨h, hne⟩ := h
  rw [tendsto_atTop_nhds] at h
  rw [eventually_atTop] at hne hs
  specialize h U hxU hU
  obtain ⟨N₁, h⟩ := h
  obtain ⟨N₂, hne⟩ := hne
  obtain ⟨N₃, hs⟩ := hs
  let N := (N₁ ⊔ N₂) ⊔ N₃
  have hN₁ : N₁ ≤ N := le_sup_left.trans le_sup_left
  have hN₂ : N₂ ≤ N := le_sup_right.trans le_sup_left
  have hN₃ : N₃ ≤ N := le_sup_right
  specialize h N hN₁
  specialize hne N hN₂
  specialize hs N hN₃
  exact ⟨f N, h, hs, hne⟩ -- ∎

-- Applies to natural numbers.
example {f : ℕ → α} (hs : ∀ᶠ n in atTop, f n ∈ s)
  (htt : Tendsto f atTop (𝓝[≠] x)) : AccPt x (𝓟 s)
  := by --
  exact AccPt.of_tendsto hs htt -- ∎

-- And this is the reason why we need (𝓝[≠] x) above, and not just (𝓝 x).
example [h₀ : Nonempty α] : ∃ (A : Set α) (x : α) (f : ℕ → α),
  (∀ᶠ n in atTop, f n ∈ A) ∧ Tendsto f atTop (𝓝 x) ∧ ¬AccPt x (𝓟 A)
  := by --
  let x := h₀.some
  refine ⟨{x}, x, fun _ ↦ x, ?_, ?_, ?_⟩
  · exact eventually_const.mpr rfl
  · exact tendsto_const_nhds
  · by_contra! h
    rw [AccPt.iff] at h
    specialize h univ isOpen_univ trivial
    rw [sdiff_self, bot_eq_empty, inter_empty] at h
    exact Set.not_nonempty_empty h -- ∎

protected theorem AccPt.t1_infinite_iff [T1Space α]
  : AccPt x (𝓟 s) ↔ ∀ u, IsOpen u → x ∈ u → (u ∩ s).Infinite
  := by --
  constructor
  · intro ha u hu hxu
    by_contra hf
    rw [not_infinite] at hf
    let t := u ∩ (s \ {x})
    have : t.Finite := hf.subset (inter_subset_inter_right _ diff_subset)
    -- `t.Finite → IsClosed t` follows from α being T1.
    have : IsOpen tᶜ := this.isClosed.isOpen_compl
    have hut : IsOpen (u ∩ tᶜ) := hu.inter this
    have hxut : x ∈ u ∩ tᶜ := by
      refine ⟨hxu, ?_⟩
      rw [mem_compl_iff, mem_inter_iff, mem_diff]
      push_neg
      intro _ _
      exact rfl
    rw [AccPt.iff] at ha
    specialize ha (u ∩ tᶜ) hut hxut
    rw [inter_right_comm, inter_compl_self t] at ha
    exact Set.not_nonempty_empty ha
  · intro hf
    rw [AccPt.iff]
    intro u hu hxu
    specialize hf u hu hxu
    obtain ⟨y, hne, hyu, hys⟩ : ∃ y ≠ x, y ∈ u ∩ s := by
      by_contra! h
      refine hf ?_
      have : u ∩ s ⊆ {x} := Set.compl_subset_compl.mp h
      exact (Set.finite_singleton x).subset this
    exact ⟨y, hyu, hys, hne⟩ -- ∎
