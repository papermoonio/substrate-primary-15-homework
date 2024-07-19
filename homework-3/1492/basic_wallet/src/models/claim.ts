class Claim {
    private _claimData: number;
    private _owner: string;
    private _address: string;

    constructor(claimData: number, owner: string, address: string) {
        this._claimData = claimData;
        this._owner = owner;
        this._address = address;
    }

    // getter and setter
    get claimData(): number {
        return this._claimData;
    }
    set claimData(value: number) {
        this._claimData = value;
    }

    get owner(): string {
        return this._owner;
    }
    set owner(value: string) {
        this._owner = value;
    }

    get address(): string {
        return this._address;
    }
    set address(value: string) {
        this._address = value;
    }
}

export default Claim;