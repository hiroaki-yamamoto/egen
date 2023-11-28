import { z } from 'zod';

export const StructArray = z.object({
  lst: z.array(z.string()),
});

export type StructArray = z.infer<typeof StructArray>;
