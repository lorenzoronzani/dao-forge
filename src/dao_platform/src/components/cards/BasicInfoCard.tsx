import { Card, CardContent } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from "@/components/ui/select"
import { LegalForm } from "@/models/entities/Dao"
import { VerticalLabeledComponent } from "@/components/labels/VerticalLabeledComponent"

type BasicInfoCardProps = {
    name: string;
    purpose: string;
    legalForm: LegalForm;
    onValueChange: (id: string, value: string) => void;
}

export const BasicInfoCard = ({ name, purpose, legalForm, onValueChange }: BasicInfoCardProps) => {
    return (
        <Card>
            <CardContent className="pt-6">
                <h3 className="text-lg font-medium mb-4">Basic Information</h3>

                <div className="grid grid-cols-1 gap-4">
                    <VerticalLabeledComponent label="Name" htmlFor="name">
                        <Input
                            id="name"
                            name="name"
                            value={name}
                            onChange={(e) => onValueChange(e.target.name, e.target.value)}
                            required
                        />
                    </VerticalLabeledComponent>

                    <VerticalLabeledComponent label="Purpose" htmlFor="purpose">
                        <Textarea
                            id="purpose"
                            name="purpose"
                            value={purpose}
                            onChange={(e) => onValueChange(e.target.name, e.target.value)}
                            className="min-h-24"
                            required
                        />
                    </VerticalLabeledComponent>

                    <VerticalLabeledComponent label="Legal form" htmlFor="legalForm">
                        <Select
                            value={legalForm || LegalForm.Association}
                            onValueChange={(value) => onValueChange('legalForm', value)}
                        >
                            <SelectTrigger>
                                <SelectValue placeholder="Select Legal Form" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectItem value={LegalForm.Association}>Association</SelectItem>
                                <SelectItem value={LegalForm.LimitedLiabilityCompany} disabled>Limited Liability Company</SelectItem>
                                <SelectItem value={LegalForm.Corporation} disabled>Corporation</SelectItem>
                            </SelectContent>
                        </Select>
                    </VerticalLabeledComponent>
                </div>
            </CardContent>
        </Card>
    )
}