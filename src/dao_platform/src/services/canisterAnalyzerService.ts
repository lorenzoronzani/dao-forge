import { Actor } from "@dfinity/agent";

export interface Parameter {
    name?: string,
    type: string,
    fields?: Parameter[],
    options?: string[]
}

export interface MethodSignature {
    name: string;
    parameters: Parameter[];
    returnType: Parameter;
}

export class CanisterAnalyzerService {
    static extractMethods(actor: any): MethodSignature[] {
        const methods: MethodSignature[] = [];

        // Get all method names from the actor
        const methodNames = this.getMethodNames(actor);

        methodNames.forEach(methodName => {
            const signature = this.extractMethodSignature(actor, methodName);
            if (signature) {
                methods.push(signature);
            }
        });

        return methods;
    }

    private static getMethodNames(actor: any): string[] {
        return Object.getOwnPropertyNames(actor).filter(name =>
            typeof actor[name] === 'function' && !name.startsWith('_')
        );
    }

    private static extractMethodSignature(actor: any, methodName: string): MethodSignature | undefined {
        const method = Actor.interfaceOf(actor)._fields.find(field => field[0] === methodName);

        if (!method) {
            return undefined;
        }

        const args = method[1].argTypes;
        const result = method[1].retTypes;

        const parameters = args.map((arg: any) => this.candidToParameter(arg));
        const returnTypeResult = this.candidToParameter(result[0]);
        const returnType = returnTypeResult ?? { type: 'unknown' };

        return {
            name: methodName,
            parameters,
            returnType: returnType
        };
    }

    private static candidToParameter(type: any): Parameter {
        // Handle Variants
        if (type.name && type.name.startsWith('variant')) {
            return {
                type: 'variant',
                options: this.extractVariantOptions(type),
            };
        }

        // Handle Records
        if (type.name === 'record' || type._fields) {
            const fields = type._fields.map((field: any) => {
                const fieldParam = this.candidToParameter(field[1]);
                return {
                    name: field[0],
                    ...fieldParam
                };
            });
            return { type: 'record', fields };
        }

        // Handle simple types
        switch (type.name) {
            case 'text':
                return { type: 'string' };
            case 'nat32':
                return { type: 'number' };
            default:
                // Handle null/unit types which appear as empty objects
                if (typeof type === 'object' && type !== null && Object.keys(type).length === 0) {
                    return { type: 'null' };
                }
                return { type: 'unknown' };
        }
    }

    private static extractVariantOptions(type: any): string[] {
        if (type.name && type.name.startsWith('variant') && type._fields) {
            return type._fields.map((field: any) => field[0]);
        }
        return [];
    }
}
export default CanisterAnalyzerService;