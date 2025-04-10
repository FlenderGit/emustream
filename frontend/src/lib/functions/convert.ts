import { SIZES, type Size } from "$lib/types/sizes";

export const convert_size_to_fontsize = (size: Size) => {
    return `${SIZES[size]* 2.2}rem`;
}