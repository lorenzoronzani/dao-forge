import { useState } from "react";

interface ExpandibleTextProps {
    text: string;
    maxLength?: number;
}

export const ExpandibleText = ({ text, maxLength = 150 }: ExpandibleTextProps) => {
    const [isExpanded, setIsExpanded] = useState<boolean>(false);

    return (
        <div className="text-sm">
            <div className="whitespace-pre-wrap">
                {isExpanded ? (
                    text
                ) : (
                    <>
                        {text.substring(0, maxLength)}...
                    </>
                )}
            </div>
            <div className="flex justify-end">
                <button
                    onClick={() => setIsExpanded(!isExpanded)}
                    className="text-xs text-slate-500 flex items-center"
                >
                    {isExpanded ? 'Show less' : 'Show more'}
                    <svg
                        className={`h-4 w-4 ml-1 transform ${isExpanded ? 'rotate-180' : ''}`}
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                    >
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
                    </svg>
                </button>
            </div>
        </div>
    );
};