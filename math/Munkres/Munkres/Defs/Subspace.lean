import Mathlib.Topology.Defs.Induced

open Set

universe u

variable {α : Type u} {X Y : Set α} [tY : TopologicalSpace Y]

@[reducible]
def Topology.Subspace (h : X ⊆ Y) : TopologicalSpace X
  := by --
  let X' := { y : Y | ↑y ∈ X }
  let φ : X ≃ X' := {
    toFun := fun ⟨x, hx⟩ ↦ ⟨⟨x, h hx⟩, hx⟩
    invFun := fun ⟨⟨x, _⟩, hx⟩ ↦ ⟨x, hx⟩
    left_inv := Function.leftInverse_iff_comp.mpr rfl
    right_inv := Function.rightInverse_iff_comp.mpr rfl
  }
  exact {
    IsOpen U := IsOpen (φ '' U)
    isOpen_univ := by
      rw [image_univ, EquivLike.range_eq_univ]
      exact isOpen_univ
    isOpen_inter s t hs ht := by
      rw [image_inter φ.injective]
      exact hs.inter ht
    isOpen_sUnion s hs := by
      rw [sUnion_eq_biUnion]
      rw [image_iUnion₂]
      exact isOpen_biUnion hs
  } -- ∎
