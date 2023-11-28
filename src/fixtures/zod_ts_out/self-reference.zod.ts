import { z } from 'zod';

export const SelfReference = z.object({
  reference: z.lazy(() => SelfReference),
});

export type SelfReference = z.infer<typeof SelfReference>;
