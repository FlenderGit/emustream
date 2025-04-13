import type { D } from "vitest/dist/chunks/reporters.d.CfRkRKN2.js";

export type Direction = 'Up' | 'Down' | 'Left' | 'Right';
export type Action = 'A' | 'B';

export type Input = Direction | Action;

type Control = string;

type Mapper<TIn extends Control, TOut extends Control> = {
    [key in TIn]: TOut
}

type ControllerKeys = Direction | Action | 'SELECT' | 'MENU';
type GameboyKeys = Direction | Action | 'Start' | 'Select';

type AlphabetKeys = 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' |
    'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z';
type NumberKeys = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
type SpecialKeys = 'Enter' | 'Escape' | 'Backspace' | 'Tab' | 'Space' | 'Shift' | 'Control' | 'Alt';
type ArrowsKeys = `Arrow${Direction}`;

type KeyboardKeys = Lowercase<AlphabetKeys> | NumberKeys | SpecialKeys | ArrowsKeys;

export enum TypeAction {
    PRESS,
    RELEASE,
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

export const map_keyboard_gb: Mapper<GameboyKeys, KeyboardKeys> = {
    "Up": "z",
    "Down": "s",
    "Left": "q",
    "Right": "d",
    "A": "j",
    "B": "k",
    "Select": "Enter",
    "Start": "Space",
}

type EventCallback<TOut> = (key: TOut) => void;

const flip = (data) => Object.fromEntries(
    Object
      .entries(data)
      .map(([key, value]) => [value, key])
    );

abstract class Controller<TIn extends Control, TOut extends Control> {

    protected events: Record<TypeAction, Array<EventCallback<TOut>>>;

    protected mapper: Mapper<TIn, TOut>;
    private buffer: Record<TOut, boolean>;

    constructor(
        mapper: Mapper<TOut, TIn>,
    ) {
        this.events = {
            [TypeAction.PRESS]: [],
            [TypeAction.RELEASE]: [],
        };

        // reverse mapper
        this.buffer = {};
        this.mapper = flip(mapper) as Mapper<TIn, TOut>;
    }

    protected dispatch_event(input: TOut, action: TypeAction) {
        console.log("Event", action, input)
        this.events[action].forEach((callback) => callback(input));
    }

    public on(action: TypeAction, callback: EventCallback<TOut>) {
        this.events[action].push(callback);
    }

}

export class KeyboardController<TOut extends Control> extends Controller<KeyboardKeys, TOut> {

    constructor(mapper: Mapper<TOut, KeyboardKeys>) {
        super(mapper);
        this.init();
    }

    private init() {
        window.addEventListener('keydown', (event) => this.handle_event(event, TypeAction.PRESS));
        window.addEventListener('keyup', (event) => this.handle_event(event, TypeAction.RELEASE));
    }

    private handle_event(event: KeyboardEvent, action: TypeAction) {
        const key = event.key as KeyboardKeys;
        if (!(key in this.mapper)) return;
        const input = this.mapper[key];
        this.dispatch_event(input, action);
    }
}