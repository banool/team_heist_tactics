
import { Batch } from "../batches/types";
import { Dye } from "../dyes/types";
import { Scent } from "../scents/types";
import { ScentCombo } from "../scent_combos/types";
import { Vessel } from "../vessels/types";
import { Wax } from "../waxes/types";

export interface DyeWithAmount {
  dye: Dye;
  amount: number;
}

export interface ScentWithAmount {
  scent: Scent;
  amount: number;
}

export interface WaxWithAmount {
  wax: Wax;
  amount: number;
}

export interface Candle {
  id: number;
  name: string | null;
  notes: string | null;
  batch: Batch;
  dyes_with_amounts: DyeWithAmount[];
  intended_scent_combo: ScentCombo;
  scents_with_amounts: ScentWithAmount[];
  vessel: Vessel;
  waxes_with_amounts: WaxWithAmount[];
}
