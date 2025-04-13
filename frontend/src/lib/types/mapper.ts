import type { GameboyKey } from '$lib/wasm/gameboy';

export type Direction = 'Up' | 'Down' | 'Left' | 'Right';
export type Action = 'A' | 'B';

export type Input = Direction | Action;

export type Control = string;

export type Mapper<TIn extends Control, TOut extends Control> = {
	[key in TIn]: TOut;
};

type ControllerKeys = Direction | Action | 'SELECT' | 'MENU';

type AlphabetKeys =
	| 'A'
	| 'B'
	| 'C'
	| 'D'
	| 'E'
	| 'F'
	| 'G'
	| 'H'
	| 'I'
	| 'J'
	| 'K'
	| 'L'
	| 'M'
	| 'N'
	| 'O'
	| 'P'
	| 'Q'
	| 'R'
	| 'S'
	| 'T'
	| 'U'
	| 'V'
	| 'W'
	| 'X'
	| 'Y'
	| 'Z';
type NumberKeys = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
type SpecialKeys = 'Enter' | 'Escape' | 'Backspace' | 'Tab' | 'Space' | 'Shift' | 'Control' | 'Alt';
type ArrowsKeys = `Arrow${Direction}`;

export type KeyboardKeys = Lowercase<AlphabetKeys> | NumberKeys | SpecialKeys | ArrowsKeys;

export enum TypeAction {
	PRESS,
	RELEASE
}

/* let map_controller_gb: Mapper<ControllerKeys, GameboyKeys> = {
    "A": "A",
    "B": "B",
    "UP": "UP",
    "DOWN": "DOWN",
    "LEFT": "LEFT",
    "RIGHT": "RIGHT",
    "SELECT": "SELECT",
    "MENU": "START",
}; */

export const map_keyboard_gb: Mapper<GameboyKey, KeyboardKeys> = {
	up: 'z',
	down: 's',
	left: 'q',
	right: 'd',
	a: 'j',
	b: 'k',
	select: 'Enter',
	start: 'Space'
};

type EventCallback<TOut> = (key: TOut) => void;

const flip = (data) => Object.fromEntries(Object.entries(data).map(([key, value]) => [value, key]));

export abstract class Controller<T extends Control> {
	// protected events: Record<TypeAction, Array<EventCallback<TOut>>>;

	/* protected mapper: Mapper<TIn, TOut>;
    private buffer: Record<TOut, boolean>; */
	public on_key_pressed: EventCallback<T>;
	public on_key_released: EventCallback<T>;

	constructor() {
		this.on_key_pressed = () => {};
		this.on_key_released = () => {};

		this.init();
	}

	abstract init(): void;
	abstract free(): void;
}

export class KeyboardController<T extends KeyboardKeys> extends Controller<T> {
	init() {
		window.addEventListener('keydown', this.handle_event.bind(this));
		window.addEventListener('keyup', this.handle_event.bind(this));
	}

	handle_event(event: KeyboardEvent) {
		const key = event.key as T;
		console.log(key, this);
		if (event.type === 'keydown') {
			this.on_key_pressed(key);
		} else if (event.type === 'keyup') {
			this.on_key_released(key);
		}
	}

	free() {
		window.removeEventListener('keydown', this.handle_event.bind(this));
		window.removeEventListener('keyup', this.handle_event.bind(this));
	}
}

export class EmptyController<T extends Control> extends Controller<T> {
	init() {}

	free() {}
}

export class Adapter<TIn extends Control, TOut extends Control> {
	public mapper: Mapper<TIn, TOut>;

	constructor(mapper: Mapper<TOut, TIn>) {
		this.mapper = flip(mapper);
	}

	public get_key = (key: TIn) => {
		return this.mapper[key];
	};
}

export class GameboyAdapter<T extends Control> extends Adapter<T, GameboyKeys> {}
