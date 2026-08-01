import Mathlib.Topology.Separation.Regular

universe u

namespace MA3209

section Definitions

variable (α : Type u) [TopologicalSpace α]

structure T₁ : Prop where
  t₁ {x y : α} : x ≠ y → ∃ U, IsOpen U ∧ x ∈ U ∧ y ∉ U

structure T₂ : Prop where
  t₂ {x y : α} : x ≠ y → ∃ U V, IsOpen U ∧ IsOpen V ∧ x ∈ U ∧ y ∈ V ∧ Disjoint U V

structure T₃ : Prop extends T₁ α where
  t₃ {x : α} {B} : x ∉ B → IsClosed B → ∃ U V, IsOpen U ∧ IsOpen V ∧ x ∈ U ∧ B ⊆ V ∧ Disjoint U V

structure T₄ : Prop extends T₁ α where
  t₄ {A B : Set α} : IsClosed A → IsClosed B → Disjoint A B →
    ∃ U V, IsOpen U ∧ IsOpen V ∧ A ⊆ U ∧ B ⊆ V ∧ Disjoint U V

end Definitions

variable {X : Type u} [TopologicalSpace X]

protected theorem T₁.iff : T₁ X ↔ T1Space X
  := by --
  rw [t1Space_iff_exists_open]
  exact ⟨fun h _ _ ↦ h.t₁, fun h ↦ { t₁ := (h ·) }⟩ -- ∎

theorem T₁.toT1 (h : T₁ X) : T1Space X := T₁.iff.mp h

protected theorem T₂.iff : T₂ X ↔ T2Space X
  := by --
  rw [t2Space_iff]
  exact ⟨fun h _ _ ↦ h.t₂, fun h ↦ { t₂ := (h ·) }⟩ -- ∎

theorem T₂.toT2 (h : T₂ X) : T2Space X := T₂.iff.mp h

protected lemma T₃.iff : T₃ X ↔ T3Space X
  := by --
  constructor
  · intro h₃
    haveI : T0Space X := h₃.toT1.t0Space
    refine { regular := ?_ }
    intro B x hB hxB
    rw [Filter.disjoint_iff]
    obtain ⟨U, V, hU, hV, hxU, hBV, h₃⟩ := h₃.t₃ hxB hB
    refine ⟨V, ?_, U, ?_, h₃.symm⟩
    · exact hV.mem_nhdsSet.mpr hBV
    · exact hU.mem_nhds hxU
  · intro h₃
    have h₁ : T₁ X := by
      rw [T₁.iff]
      exact instT1SpaceOfT0SpaceOfR0Space
    exact {
      t₁ := h₁.t₁
      t₃ {x B} hxB hB := by
        replace h₃ := h₃.toRegularSpace
        rw [regularSpace_iff] at h₃
        specialize h₃ hB hxB
        rw [Filter.disjoint_iff] at h₃
        obtain ⟨V', hV', U', hU', h₃⟩ := h₃
        rw [mem_nhds_iff] at hU'
        rw [mem_nhdsSet_iff_exists] at hV'
        obtain ⟨U, hUU', hU, hxU⟩ := hU'
        obtain ⟨V, hV, hBV, hVV'⟩ := hV'
        refine ⟨U, V, hU, hV, hxU, hBV, ?_⟩
        exact Set.disjoint_of_subset hUU' hVV' h₃.symm
    } -- ∎

theorem T₃.toT3 (h : T₃ X) : T3Space X := T₃.iff.mp h

protected lemma T₄.iff : T₄ X ↔ T4Space X
  := by --
  constructor
  · intro h₄
    haveI : T1Space X := h₄.toT1
    refine { normal := ?_ }
    intro A B hA hB hd
    replace h₄ := h₄.t₄ hA hB hd
    obtain ⟨U, V, hU, hV, hAU, hBV, h₄⟩ := h₄
    rw [separatedNhds_iff_disjoint, Filter.disjoint_iff]
    refine ⟨U, ?_, V, ?_, h₄⟩
    · exact hU.mem_nhdsSet.mpr hAU
    · exact hV.mem_nhdsSet.mpr hBV
  · intro h₄
    have h₁ : T₁ X := by
      rw [T₁.iff]
      exact instT1SpaceOfT0SpaceOfR0Space
    exact {
      t₁ := h₁.t₁
      t₄ {A B} hA hB hd := by
        replace h₄ := h₄.normal A B hA hB hd
        rw [separatedNhds_iff_disjoint, Filter.disjoint_iff] at h₄
        obtain ⟨U', hU', V', hV', h₄⟩ := h₄
        rw [mem_nhdsSet_iff_exists] at hU' hV'
        obtain ⟨U, hU, hAU, hUU'⟩ := hU'
        obtain ⟨V, hV, hBV, hVV'⟩ := hV'
        refine ⟨U, V, hU, hV, hAU, hBV, ?_⟩
        exact Set.disjoint_of_subset hUU' hVV' h₄
    } -- ∎

theorem T₄.toT4 (h : T₄ X) : T4Space X := T₄.iff.mp h

end MA3209
