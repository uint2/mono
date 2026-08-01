import Mathlib.Topology.Order

universe u

variable {α : Type u} [t₁ : TopologicalSpace α] [t₂ : TopologicalSpace α]

private def TopologicalSpace.opens (t : TopologicalSpace α) : Set (Set α) := { U | @IsOpen α t U }

-- If t₁ ⊇ t₂, then we say that t₁ is *finer* than t₂.
private def TopologicalSpace.isFiner (t₁ t₂ : TopologicalSpace α) : Prop := t₁ ≤ t₂

example : t₁.isFiner t₂ ↔ ∀ U, @IsOpen α t₂ U → @IsOpen α t₁ U := by
  rw [TopologicalSpace.isFiner]
  exact ⟨(·), (·)⟩

-- We say: t₂ is finer than t₁, and that t₁ is coarser than t₂.
example : t₂ ≤ t₁ ↔ ∀ U, @IsOpen α t₁ U → @IsOpen α t₂ U := ⟨(·), (·)⟩

-- Mathlib kinda flips the two. So the finer the topology, the "smaller" it is
-- in Mathlib.
example : t₂ ≤ t₁ → t₁.opens ⊆ t₂.opens := (·)
example : t₂.isFiner t₁ → t₁.opens ⊆ t₂.opens := (·)
