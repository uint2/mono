import Mathlib.Topology.Order

import Munkres.Mathlib.Subtype

universe u

variable {α : Type u} [TopologicalSpace α]

section Induced

variable (X : Set α) {A : Set α}

/-- Matches Munkres' definition of a set being open in a topological subspace. -/
theorem isOpen_induced_iff₂ (A : Set X) : @IsOpen X _ A ↔ ∃ U, IsOpen U ∧ A = X ∩ U
  := by --
  rw [isOpen_induced_iff]
  simp only [Subtype.preimage_val_eq_iff] -- ∎

/-- Matches Munkres' definition of a set being closed in a topological subspace. -/
theorem isClosed_induced_iff₂ (A : Set X) : @IsClosed X _ A ↔ ∃ U, IsClosed U ∧ A = X ∩ U
  := by --
  rw [isClosed_induced_iff]
  simp only [Subtype.preimage_val_eq_iff] -- ∎

theorem isOpen_induced_iff₃ :
  @IsOpen X _ (Subtype.val ⁻¹' A) ↔ ∃ U, IsOpen U ∧ X ∩ A = X ∩ U
  := by --
  rw [isOpen_induced_iff]
  simp only [Subtype.preimage_val_eq_iff, Subtype.image_preimage_coe] -- ∎

theorem isClosed_induced_iff₃ :
  @IsClosed X _ (Subtype.val ⁻¹' A) ↔ ∃ U, IsClosed U ∧ X ∩ A = X ∩ U
  := by --
  rw [isClosed_induced_iff]
  simp only [Subtype.preimage_val_eq_iff, Subtype.image_preimage_coe] -- ∎

end Induced
