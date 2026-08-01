import Munkres.Mathlib.Disjoint
import Munkres.Mathlib.Prelude
import Munkres.Closure.Subtype

universe u

variable {α : Type u} [TopologicalSpace α]

section SubspaceTopology

variable {X : Set α} {A B : Set X}

/-- If A and B are disjoint and A is closed, then
A = closure A ∩ X ↔ Disjoint (closure A) B,
where both closures are in the superspace. -/
theorem Disjoint.eq_closure_iff_disjoint (hAB : Disjoint A B) (hA : IsClosed A)
  : A = closure (Subtype.val '' A) ∩ X ↔
  Disjoint (closure (Subtype.val '' A)) (Subtype.val '' B)
  := by --
  rw [<-Set.image_val_inter_self_right_eq_coe (D := B), <-disjoint_assoc₂]
  refine ⟨(· ▸ Set.disjoint_image_subtype_iff.mpr hAB), ?_⟩
  intro h'
  rw [<-closure_subtype₂ A, Set.image_val_inj]
  exact hA.closure_eq.symm -- ∎

end SubspaceTopology
