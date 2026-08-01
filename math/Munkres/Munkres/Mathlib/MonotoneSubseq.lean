import Mathlib.Data.Set.Finite.Lemmas
import Mathlib.Order.Interval.Finset.Nat

universe u

variable {α : Type u} [LinearOrder α]

variable {x : ℕ → α}

private def IsPeak (x : ℕ → α) (n : ℕ) : Prop := ∀ m ≥ n, x m ≤ x n

private structure Node (N : ℕ) where
  n : ℕ
  ge' : n ≥ N

private noncomputable def ζ_mono {x : ℕ → α} {N : ℕ}
  (h : ∀ n ≥ N, ∃ m > n, x m > x n) : ℕ → Node N
  | 0 => { n := N, ge' := le_rfl }
  | n + 1 => by
    let p := ζ_mono h n
    let m := Nat.find <| h p.n p.ge'
    have hm := (Nat.find_spec (h p.n p.ge')).1
    exact { n := m, ge' := p.ge'.trans hm.le }

private noncomputable def ζ_anti {x : ℕ → α} (h : ∀ N, ∃ n, IsPeak x n ∧ N < n) : ℕ → ℕ
  | 0 => (h 0).choose
  | n + 1 => (h (ζ_anti h n)).choose

private lemma exists_mono_subseq {x : ℕ → α}
  : ∃ φ : ℕ → ℕ, StrictMono φ ∧ (Monotone (x ∘ φ) ∨ Antitone (x ∘ φ))
  := by --
  let P := { n | IsPeak x n }
  if hPf : P.Finite then
    obtain ⟨N, hN : ∀ n ∈ P, n ≤ N⟩ := Set.exists_upper_bound_image P id hPf
    have hδ : ∀ n ≥ N + 1, ∃ m > n, x m > x n := by
      intro n hn
      by_contra! h'
      refine (hN n ?_).not_gt hn
      intro m hm
      rcases hm.eq_or_lt with heq | hlt
      · subst heq
        exact le_rfl
      · exact h' m hlt
    let φ (n : ℕ) := ζ_mono hδ n
    refine ⟨fun n ↦ (φ n).n, ?_, Or.inl ?_⟩
    · refine strictMono_nat_of_lt_succ ?_
      intro n
      specialize hδ (φ n).n (φ n).ge'
      exact (Nat.find_spec hδ).1
    · refine monotone_nat_of_le_succ ?_
      intro n
      specialize hδ (φ n).n (φ n).ge'
      exact (Nat.find_spec hδ).2.le
  else -- P.Infinite
    change P.Infinite at hPf
    have h := not_bddAbove_iff.mp (hPf : P.Infinite).not_bddAbove
    let φ (n : ℕ) : ℕ := ζ_anti h n
    refine ⟨φ, ?_, Or.inr ?_⟩
    · refine strictMono_nat_of_lt_succ ?_
      intro n
      exact (h (φ n)).choose_spec.2
    · refine antitone_nat_of_succ_le ?_
      intro n
      have hp : IsPeak x (φ n) := match n with
      | 0 => (h 0).choose_spec.1
      | n + 1 => (h (φ n)).choose_spec.1
      have : φ n < φ (n + 1) := (h (φ n)).choose_spec.2
      exact hp (φ (n + 1)) this.le -- ∎
