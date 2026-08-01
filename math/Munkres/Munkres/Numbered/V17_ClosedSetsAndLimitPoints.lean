import Munkres.Closure.Subtype
import Munkres.Mathlib.AccPt.Basic
import Munkres.Mathlib.Disjoint
import Munkres.Subtype.Topology

open Set Topology Filter TopologicalSpace

universe u v

variable {α : Type u} {β : Type v} [TopologicalSpace α]

section S₂
--* Theorem 17.2
-- Let Y be a subspace of X. Then a set A is closed in Y ↔ it equals the
-- intersection of a closed set of X with Y.
variable {Y : Set α}

private lemma ℓ₁ {A : Set Y} : IsClosed A ↔ ∃ G, IsClosed G ∧ Subtype.val '' A = Y ∩ G
  := by --
  rw [isClosed_induced_iff]
  simp only [Subtype.preimage_val_eq_iff] -- ∎

private lemma ℓ₂ {A : Set α} : IsClosedIn A Y ↔ ∃ G, IsClosed G ∧ A = G ∩ Y
  := by --
  exact IsClosedIn.iff -- ∎
end S₂

section S₃
--* Theorem 17.3
-- Let Y be a subspace of X. If A is closed in Y and Y is closed in X, then A
-- is closed in X .
variable {Y : Set α}

private lemma ℓ₃ {A : Set Y} : IsClosed A → IsClosed Y → IsClosed (Subtype.val '' A)
  := by --
  exact IsClosed.trans -- ∎

private lemma ℓ₄ {A : Set α} : IsClosedIn A Y → IsClosed Y → IsClosed A
  := by --
  exact IsClosedIn.trans -- ∎
end S₃

section S₄
--* Theorem 17.4
-- Let Y be a subspace of X, A ⊆ Y, let Ā denote the closure of A in X. Then
-- the closure of A in Y equals Ā ∩ Y.
variable {Y : Set α}

--* Theorem 17.4: If A ⊆ Y ⊆ X, the closure of A in Y = (closure A in X) ∩ Y.
private lemma ℓ₅ {A : Set Y} : closure A = closure (A : Set α) ∩ Y
  := by --
  exact closure_subtype₂ A -- ∎

--* Theorem 17.4: If A ⊆ Y ⊆ X, the closure of A in Y = (closure A in X) ∩ Y.
private lemma ℓ₆ {A : Set α} {hAY : A ⊆ Y} : closure (A.as Y) = closure A ∩ Y
  := by --
  exact closure_subtype₃ hAY -- ∎

end S₄

section S₅
--* Theorem 17.5: equivalences for mem_closure
variable {A : Set α} {x : α} {B : Set (Set α)}

--* Theorem 17.5(a): equivalence for mem_closure. Mathlib's `mem_closure_iff`.
private lemma ℓ₇ : x ∈ closure A ↔ ∀ U, IsOpen U → x ∈ U → (U ∩ A).Nonempty
  := by --
  refine not_iff_not.mp ⟨?_, ?_⟩
  · intro h
    push_neg
    refine ⟨(closure A)ᶜ, isClosed_closure.isOpen_compl, h, ?_⟩
    refine Disjoint.inter_eq ?_
    rw [disjoint_compl_left_iff_subset]
    exact subset_closure
  · intro h
    push_neg at h
    obtain ⟨U, hU, hxU, hd⟩ := h
    have : IsClosed Uᶜ := hU.isClosed_compl
    have : A ⊆ Uᶜ := (Disjoint.tfae.out 0 3).mp hd
    have : closure A ⊆ Uᶜ := (hU.isClosed_compl.closure_subset_iff).mpr this
    exact (this · hxU) -- ∎
-- The Mathlib way.
private lemma ℓ₈ : x ∈ closure A ↔ ∀ U, IsOpen U → x ∈ U → (U ∩ A).Nonempty
  := mem_closure_iff

--* Theorem 17.5(b). Mathlib's `IsTopologicalBasis.mem_closure_iff`.
private lemma ℓ₉ (hB : IsTopologicalBasis B) : x ∈ closure A ↔ ∀ b ∈ B, x ∈ b → (b ∩ A).Nonempty
  := by --
  rw [mem_closure_iff]
  constructor
  · intro h b hbB hxb
    have : IsOpen b := hB.isOpen hbB
    exact h b this hxb
  · intro h u hu hxu
    rw [hB.isOpen_iff] at hu
    obtain ⟨b, hbB, hxb, hbu⟩ := hu x hxu
    exact (h b hbB hxb).mono (inter_subset_inter_left A hbu) -- ∎
-- The Mathlib way.
private lemma ℓ₁₀ (hB : IsTopologicalBasis B) : x ∈ closure A ↔ ∀ b ∈ B, x ∈ b → (b ∩ A).Nonempty
  := hB.mem_closure_iff

end S₅

section S₆
--* Theorem 17.6: Ā = A ∪ A'
private lemma ℓ₁₁ {A : Set α} : closure A = A ∪ { x | AccPt x (𝓟 A) }
  := by --
  exact AccPt.union_eq_closure.symm -- ∎

end S₆

section S₇
--* Theorem 17.7: a set is closed ↔ it contains all its limit points.
private lemma ℓ₁₂ {s : Set α} : (∀ x, AccPt x (𝓟 s) → x ∈ s) ↔ IsClosed s
  := by --
  simp only [isClosed_iff_clusterPt, clusterPt_principal]
  refine ⟨?_, (· · <| Or.inr ·)⟩
  intro h x hxs
  specialize h x
  cases hxs with
  | inl h => exact h
  | inr h' => exact h h' -- ∎

end S₇

section S₈
variable {s : Set α} {x₀ : α}
--* Theorem 17.8: In a T1 space, finite sets are closed.
-- Munkres starts with T2 because he uses the fact that finite sets are closed
-- to define T1.
private lemma ℓ₁₃ [T2Space α] : s.Finite → IsClosed s
  := by --
  intro hs
  induction s, hs using Set.Finite.induction_on with
  | empty => exact isClosed_empty
  | @insert x₀ s hxs hsF hs =>
    refine IsClosed.union ?_ hs
    suffices heq : {x₀} = closure {x₀} by
      change IsClosed {x₀}
      rw [heq]
      exact isClosed_closure
    refine Subset.antisymm subset_closure ?_
    rw [<-compl_subset_compl]
    intro x (hx : x ≠ x₀)
    have h₂ : T2Space α := by infer_instance
    obtain ⟨U, V, hU, hV, hxU, hx₀V, hd⟩ := h₂.t2 hx
    change x ∉ closure {x₀}
    rw [mem_closure_iff]
    push_neg
    refine ⟨U, hU, hxU, Disjoint.inter_eq ?_⟩
    exact hd.mono_right (singleton_subset_iff.mpr hx₀V) -- ∎
-- The Mathlib way.
private lemma ℓ₁₄ [T2Space α] : s.Finite → IsClosed s := Set.Finite.isClosed
-- In fact, it suffices that the space is T1. In T1 spaces, singleton sets are
-- closed.
private lemma ℓ₁₅ [T1Space α] : s.Finite → IsClosed s := Set.Finite.isClosed
private lemma ℓ₁₆ [T1Space α] : IsClosed {x₀} := isClosed_singleton

end S₈

section S₉

variable [T1Space α] {s : Set α} {x : α}
--* Theorem 17.9
-- x is a limit point of A in a T1 space iff every open set that contains x
-- also contains infinitely many points of A.
private lemma ℓ₁₇ : AccPt x (𝓟 s) ↔ ∀ u, IsOpen u → x ∈ u → (u ∩ s).Infinite
  := by --
  exact AccPt.t1_infinite_iff -- ∎
end S₉

section S₁₀
variable [T2Space α] {f : ℕ → α} {x y : α}

--* Theorem 17.10
-- In a T2 space, the point which any sequence converges to is unique.
private lemma ℓ₁₈ : Tendsto f atTop (𝓝 x) → Tendsto f atTop (𝓝 y) → x = y
  := by --
  intro hx_tt hy_tt
  -- Set the T2 stage first.
  by_contra hne
  have h₂ : T2Space α := by infer_instance
  obtain ⟨U, V, hU, hV, hxU, hyV, hd⟩ := h₂.t2 hne
  -- Because `f → x` (and `f → y`), this is stronger than just saying that near
  -- x (and y) we can find infintely many values of `f n`. It's saying that we
  -- can find ALL but a finite number of those values.
  rw [tendsto_atTop'] at hx_tt hy_tt
  -- specialize to the disjoint neighborhoods brought to you by T₂.
  obtain ⟨Nx, hNx : ∀ n ≥ Nx, f n ∈ U⟩ := hx_tt U (hU.mem_nhds hxU)
  obtain ⟨Ny, hNy : ∀ n ≥ Ny, f n ∈ V⟩ := hy_tt V (hV.mem_nhds hyV)
  -- But see how we can already easily find an element that's in both sets. This
  -- contradicts the fact that they are disjoint.
  let N := max Nx Ny
  specialize hNx N (le_max_left _ _)
  specialize hNy N (le_max_right _ _)
  have h₀ : (U ∩ V).Nonempty := ⟨f N, hNx, hNy⟩
  exact h₀.not_disjoint hd -- ∎

-- A quick remark on the strength of convergence.
example : Tendsto f atTop (𝓝[≠] x) → Tendsto f atTop (𝓝 x)
  := fun h ↦ fun _ ↦ (h <| nhdsWithin_le_nhds ·)

private lemma ℓ₁₉ : Tendsto f atTop (𝓝[≠] x) → Tendsto f atTop (𝓝[≠] y) → x = y
  := by --
  intro hx_tt hy_tt
  replace hx_tt : Tendsto f atTop (𝓝 x) := fun u hu ↦ hx_tt (nhdsWithin_le_nhds hu)
  replace hy_tt : Tendsto f atTop (𝓝 y) := fun u hu ↦ hy_tt (nhdsWithin_le_nhds hu)
  exact ℓ₁₈ hx_tt hy_tt -- ∎

end S₁₀

section S₁₁
variable [TopologicalSpace β] [hα : T2Space α] [hβ : T2Space β] {X : Set α}

--* Theorem 17.11(a): product of T2 space is again T2.
example : T2Space (α × β)
  := by --
  refine { t2 := fun ⟨x₁, y₁⟩ ⟨x₂, y₂⟩ hne => ?_ }
  replace hne : x₁ ≠ x₂ ∨ y₁ ≠ y₂ := by
    by_contra! h
    exact hne (Prod.mk_inj.mpr h)
  rcases hne with hne_x | hne_y
  · obtain ⟨U₁, U₂, hU₁, hU₂, hx₁, hx₂, hd⟩ := hα.t2 hne_x
    use U₁ ×ˢ univ, U₂ ×ˢ univ
    refine ⟨?_, ?_, ?_, ?_, ?_⟩
    · exact hU₁.prod isOpen_univ
    · exact hU₂.prod isOpen_univ
    · exact ⟨hx₁, trivial⟩
    · exact ⟨hx₂, trivial⟩
    · exact Disjoint.set_prod_left hd univ univ
  · obtain ⟨U₁, U₂, hU₁, hU₂, hx₁, hx₂, hd⟩ := hβ.t2 hne_y
    use univ ×ˢ U₁, univ ×ˢ U₂
    refine ⟨?_, ?_, ?_, ?_, ?_⟩
    · exact isOpen_univ.prod hU₁
    · exact isOpen_univ.prod hU₂
    · exact ⟨trivial, hx₁⟩
    · exact ⟨trivial, hx₂⟩
    · exact Disjoint.set_prod_right hd univ univ -- ∎

--* Theorem 17.11(b): subspace of T2 space is again T2.
example : T2Space X
  := by --
  refine { t2 := ?_ }
  intro x y hne
  replace hne : x.val ≠ y.val := Subtype.coe_ne_coe.mpr hne
  obtain ⟨U, V, hU, hV, hxU, hyV, hd⟩ := hα.t2 hne
  refine ⟨U.as X, V.as X, ?_, ?_, hxU, hyV, hd.as X⟩
  · rw [isOpen_induced_iff]
    exact ⟨U, hU, rfl⟩
  · rw [isOpen_induced_iff]
    exact ⟨V, hV, rfl⟩ -- ∎

-- Mathlib's ways
example : T2Space (α × β) ∧ T2Space X := ⟨Prod.t2Space, instT2SpaceSubtype⟩
end S₁₁
