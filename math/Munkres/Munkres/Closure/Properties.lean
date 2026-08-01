import Mathlib.Algebra.Order.Archimedean.Basic
import Mathlib.Algebra.Order.Ring.Star
import Mathlib.Topology.Bases
import Munkres.Mathlib.Disjoint

open Topology Filter TopologicalSpace

universe u

variable {α : Type u} [TopologicalSpace α] {A : Set α} {x : α} {B : Set (Set α)}

--* Theorem 17.5(a): equivalence for mem_closure. Mathlib's `mem_closure_iff`.
example : x ∈ closure A ↔ ∀ U, IsOpen U → x ∈ U → (U ∩ A).Nonempty
  := by --
  refine not_iff_not.mp ⟨?_, ?_⟩
  · intro h
    push_neg
    refine ⟨(closure A)ᶜ, isClosed_closure.isOpen_compl, h, ?_⟩
    refine Disjoint.inter_eq ?_
    rw [Set.disjoint_compl_left_iff_subset]
    exact subset_closure
  · intro h
    push_neg at h
    obtain ⟨U, hU, hxU, hd⟩ := h
    have : IsClosed Uᶜ := hU.isClosed_compl
    have : A ⊆ Uᶜ := (Disjoint.tfae.out 0 3).mp hd
    have : closure A ⊆ Uᶜ := (hU.isClosed_compl.closure_subset_iff).mpr this
    exact (this · hxU) -- ∎

example : x ∈ closure A ↔ ∀ U, IsOpen U → x ∈ U → (U ∩ A).Nonempty
  := by --
  rw [mem_closure_iff] -- ∎

--* Theorem 17.5(b). Mathlib's `IsTopologicalBasis.mem_closure_iff`.
example (hB : IsTopologicalBasis B) : x ∈ closure A ↔ ∀ b ∈ B, x ∈ b → (b ∩ A).Nonempty
  := by --
  rw [mem_closure_iff]
  constructor
  · intro h b hbB hxb
    have : IsOpen b := hB.isOpen hbB
    exact h b this hxb
  · intro h u hu hxu
    rw [hB.isOpen_iff] at hu
    obtain ⟨b, hbB, hxb, hbu⟩ := hu x hxu
    exact (h b hbB hxb).mono (Set.inter_subset_inter_left A hbu) -- ∎

example (hB : IsTopologicalBasis B) : x ∈ closure A ↔ ∀ b ∈ B, x ∈ b → (b ∩ A).Nonempty
  := by --
  exact hB.mem_closure_iff -- ∎

section Tendsto
-- If a sequence converges to a point x, then that point x is in the closure of
-- the set containing that sequence.
variable (f : ℕ → α) (h : Tendsto f atTop (𝓝 x))

example (hf : ∀ i, f i ∈ A) : x ∈ closure A
  := by --
  have : ∀ᶠ n in atTop, f n ∈ A := Eventually.of_forall hf
  exact mem_closure_of_tendsto h this -- ∎

example : x ∈ closure (Set.range f)
  := by --
  have : ∀ᶠ n in atTop, f n ∈ Set.range f := Eventually.of_forall Set.mem_range_self
  exact mem_closure_of_tendsto h this -- ∎

end Tendsto
