import Mathlib.Topology.Defs.Filter

universe u

variable {α : Type u}

-- Conforms to Munkres' idea of a set being open in a subspace.
structure IsOpenIn (A X : Set α) [TopologicalSpace X] : Prop where
  isOpen' : @IsOpen X _ (Subtype.val ⁻¹' A)
  subset' : A ⊆ X

-- Conforms to Munkres' idea of a set being closed in a subspace.
structure IsClosedIn (A X : Set α) [TopologicalSpace X] : Prop where
  isClosed' : @IsClosed X _ (Subtype.val ⁻¹' A)
  subset' : A ⊆ X

-- Conforms to Munkres' idea of a set being compact in a subspace.
structure IsCompactIn (A X : Set α) [TopologicalSpace X] : Prop where
  isCompact' : @IsCompact X _ (Subtype.val ⁻¹' A)
  subset' : A ⊆ X
