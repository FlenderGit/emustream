import type { Game } from "$lib/types/entities";

export function chunk_list<T>(list: T[], chunkSize: number): T[][] {
  const chunks: T[][] = [];
  for (let i = 0; i < list.length; i += chunkSize) {
    chunks.push(list.slice(i, i + chunkSize));
  }
  return chunks;
}

export function extract_regions(game: Game): string[] {
  const regions = new Set<string>();
  game.releases.forEach((r) => {
    regions.add(r.region);
  });
  return Array.from(regions).sort();
}

export function extract_platforms(game: Game): string[] {
  const platforms = new Set<string>();
  game.releases.forEach((r) => {
    platforms.add(r.platform);
  });
  return Array.from(platforms).sort();
}