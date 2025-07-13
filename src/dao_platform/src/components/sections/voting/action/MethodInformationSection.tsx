import { MethodSignature, Parameter } from "@/services/canisterAnalyzerService";
import { Code } from "lucide-react";

interface MethodInformationSectionProps {
    method: MethodSignature;
}


export const MethodInformationSection = ({ method }: MethodInformationSectionProps) => {
    const formatSingleParam = (param: Parameter): string => {
        const namePart = param.name ? `${param.name}: ` : '';

        if (param.type === 'record') {
            const fieldsString = param.fields?.map(formatSingleParam).join(', ');
            return `${namePart}record { ${fieldsString} }`;
        }

        if (param.type === 'variant') {
            const optionsString = param.options?.join(', ');
            return `${namePart}variant { ${optionsString} }`;
        }

        return `${namePart}${param.type}`;
    };

    const formatParams = (params: Parameter[]): string => {
        return params.map(formatSingleParam).join(', ');
    }

    return (
        <div className="p-3 bg-slate-50 rounded-lg">
            <div className="flex items-center gap-2 mb-2">
                <Code className="h-4 w-4" />
                <span className="font-medium text-sm">Method Signature</span>
            </div>
            <code className="text-xs bg-white p-2 rounded border block">
                {method.name}({formatParams(method.parameters)}) → {method.returnType.type}
            </code>
        </div>
    );
}

export default MethodInformationSection;

