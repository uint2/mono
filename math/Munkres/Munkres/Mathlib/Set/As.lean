import Mathlib.Data.Set.BooleanAlgebra
import Mathlib.Order.CompletePartialOrder
import Munkres.Mathlib.Subtype

universe u

variable {α : Type u}

@[simp]
abbrev Set.as (s β : Set α) : Set β := { x : β | ↑x ∈ s }

theorem Set.as_subtype_subset_eq {A X : Set α}
  : A ⊆ X → Subtype.val '' (A.as X) = A
  := Subtype.coe_image_of_subset

theorem Set.as_eq_inter (s β : Set α) : s.as β = s ∩ β
  := by --
  rw [Set.inter_comm]
  exact Subtype.eq_inter -- ∎

theorem Set.as_eq_inter' (s β : Set α) : s.as β = β ∩ s
  := by --
  exact Subtype.eq_inter -- ∎

theorem Set.eq_of_as {s t β : Set α} (hs : s ⊆ β) (ht : t ⊆ β) : s.as β = t ∩ β → s = t
  := by --
  rw [Set.as_eq_inter]
  intro heq
  rw [<-left_eq_inter] at hs ht
  rw [<-hs, <-ht] at heq
  exact heq -- ∎

theorem Set.compl_as (s β : Set α) : (β \ s).as β = (s.as β)ᶜ
  := by --
  refine le_antisymm ?_ (⟨·.prop, ·⟩)
  intro x ⟨hxβ, hxs⟩
  exact hxs -- ∎

-- No need for a theorem for this one.
example {s β : Set α} : (sᶜ).as β = (s.as β)ᶜ := rfl
