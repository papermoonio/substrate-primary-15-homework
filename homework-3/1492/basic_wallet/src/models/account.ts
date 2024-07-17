class Account {
    private _name: string | undefined;
    private _address: string;
    private _freeBalance: number | bigint;

    constructor(name: string | undefined, address: string, freeBalance: number | bigint) {
        this._name = name;
        this._address = address;
        this._freeBalance = freeBalance;
    }

    // getter and setter
    get name(): string | undefined {
        return this._name;
    }
    set name(value: string | undefined) {
        this._name = value;
    }

    get address(): string {
        return this._address;
    }
    set address(value: string) {
        this._address = value;
    }

    get freeBalance(): number | bigint {
        return this._freeBalance;
    }
    set freeBalance(value: number | bigint) {
        this._freeBalance = value;
    }
}

export default Account;