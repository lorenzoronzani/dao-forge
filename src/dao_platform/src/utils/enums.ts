type CandidConvertedEnum = {
    [key: string]: null;
}

export const candidToEnum = <T extends Record<string, number | string>>(
    candidValue: CandidConvertedEnum,
    enumType: T
): T[keyof T] => {
    const key = Object.keys(candidValue)[0];

    if (!(key in enumType)) {
        throw new Error(`Key "${key}" not found in enum ${enumType}`);
    }

    return enumType[key as keyof T];
}