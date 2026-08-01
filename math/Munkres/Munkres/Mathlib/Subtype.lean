import Mathlib.Data.Set.Image
import Mathlib.Data.Set.Notation

universe u

variable {α : Type u}

section Examples
open Subtype
variable {X U : Set α}

/-- Reminder of other forms that `Subtype.val ⁻¹' U` might take! -/
example : Subtype.val⁻¹' U = { x : X | ↑x ∈ U } := rfl
example : (↑) '' ((↑) ⁻¹' U : Set X) = X ∩ U := image_preimage_val X U
example : (↑) '' ((↑) ⁻¹' U : Set X) = X ∩ U := image_preimage_coe X U
example : val '' (val ⁻¹' U : Set X) = X ∩ U := image_preimage_val X U
example : val '' (val ⁻¹' U : Set X) = X ∩ U := image_preimage_coe X U

example (h : U ⊆ X) : val '' { x : X | ↑x ∈ U } = U := coe_image_of_subset h

end Examples

section Lemmas
variable {X U : Set α}

theorem Subtype.preimage_val_eq_iff (A : Set X)
  : val ⁻¹' U = A ↔ val '' A = X ∩ U
  := by --
  constructor
  · intro h
    subst h
    exact image_preimage_val X U
  · intro h
    rw [<-preimage_coe_self_inter, <-h, Set.preimage_image_eq]
    exact val_injective -- ∎

theorem Subtype.preimage_coe_eq_iff (A : Set X)
  : (↑) ⁻¹' U = A ↔ (↑) '' A = X ∩ U
  := by --
  exact preimage_val_eq_iff A -- ∎

theorem Subtype.eq_inter : { x : X | ↑x ∈ U } = X ∩ U
  := by --
  exact (preimage_val_eq_iff _).mp rfl -- ∎

theorem Subtype.mem_iff (A : Set X) (x : X) : x ∈ A ↔ x.val ∈ (↑) '' A
  := by --
  refine ⟨(⟨x, ·, rfl⟩), ?_⟩
  intro ⟨y, hy, heq⟩
  rw [SetCoe.ext heq] at hy
  exact hy -- ∎

theorem Subtype.set_eq (A : Set X) : { x : X | ↑x ∈ (A : Set α) } = A
  := by --
  refine le_antisymm ?_ (⟨·, ·, rfl⟩)
  intro x ⟨y, hy, heq⟩
  rw [SetCoe.ext heq] at hy
  exact hy -- ∎

theorem Subtype.compl : (val ⁻¹' U : Set X)ᶜ = val ⁻¹' (X \ U)
  := by --
  rw [Set.preimage_diff, coe_preimage_self]
  exact Set.compl_eq_univ_diff (val ⁻¹' U) -- ∎

end Lemmas
