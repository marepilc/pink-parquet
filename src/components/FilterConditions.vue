<script setup lang="ts">
import { Condition } from '~/types/app-types'

const props = defineProps<{
    modelValue: Condition
    dtype: string
}>()

const emit = defineEmits(['update:modelValue'])

const selectedCondition = ref<Condition | null>(null)

watch(
    () => props.modelValue,
    (newValue) => {
        selectedCondition.value = newValue
    }
)

function updateCondition(condition: Condition) {
    if (condition === selectedCondition.value) {
        selectedCondition.value = null
        emit('update:modelValue', null)
    } else {
        selectedCondition.value = condition
        emit('update:modelValue', condition)
    }
}
</script>

<template>
    <SelectButton
        v-if="props.dtype === 'String'"
        :modelValue="selectedCondition"
        :options="[
            Condition.equals,
            Condition.contains,
            Condition.containsCaseInsensitive,
            Condition.different,
            Condition.isNull,
            Condition.isNotNull,
        ]"
        @update:modelValue="updateCondition"
    >
        <template #option="slotProps">
            <IconEq
                v-if="slotProps.option === Condition.equals"
                class="h-4 w-4"
            />
            <IconContainsSens
                v-if="slotProps.option === Condition.contains"
                class="h-4 w-4"
            />
            <IconContainsInsens
                v-if="slotProps.option === Condition.containsCaseInsensitive"
                class="h-4 w-4"
            />
            <IconNeq
                v-if="slotProps.option === Condition.different"
                class="h-4 w-4"
            />
            <IconNull
                v-if="slotProps.option === Condition.isNull"
                class="h-4 w-4"
            />
            <IconNotNull
                v-if="slotProps.option === Condition.isNotNull"
                class="h-4 w-4"
            />
        </template>
    </SelectButton>
    <SelectButton
        v-else-if="['Categorical', 'Enum'].includes(props.dtype)"
        :modelValue="selectedCondition"
        :options="[
            Condition.equals,
            Condition.different,
            Condition.isNull,
            Condition.isNotNull,
        ]"
        @update:modelValue="updateCondition"
    >
        <template #option="slotProps">
            <IconEq
                v-if="slotProps.option === Condition.equals"
                class="h-4 w-4"
            />
            <IconNeq
                v-if="slotProps.option === Condition.different"
                class="h-4 w-4"
            />
            <IconNull
                v-if="slotProps.option === Condition.isNull"
                class="h-4 w-4"
            />
            <IconNotNull
                v-if="slotProps.option === Condition.isNotNull"
                class="h-4 w-4"
            />
        </template>
    </SelectButton>
    <SelectButton
        v-else-if="props.dtype === 'Boolean'"
        :modelValue="selectedCondition"
        :options="[Condition.eq, Condition.isNull, Condition.isNotNull]"
        @update:modelValue="updateCondition"
    >
        <template #option="slotProps">
            <IconEq v-if="slotProps.option === Condition.eq" class="h-4 w-4" />
            <IconNull
                v-if="slotProps.option === Condition.isNull"
                class="h-4 w-4"
            />
            <IconNotNull
                v-if="slotProps.option === Condition.isNotNull"
                class="h-4 w-4"
            />
        </template>
    </SelectButton>
    <SelectButton
        v-else-if="props.dtype === 'Datetime'"
        :modelValue="selectedCondition"
        :options="[
            Condition.lt,
            Condition.le,
            Condition.neq,
            Condition.ge,
            Condition.gt,
            Condition.between,
            Condition.isNull,
            Condition.isNotNull,
        ]"
        @update:modelValue="updateCondition"
    >
        <template #option="slotProps">
            <IconLt v-if="slotProps.option === Condition.lt" class="h-4 w-4" />
            <IconLe v-if="slotProps.option === Condition.le" class="h-4 w-4" />
            <IconEq v-if="slotProps.option === Condition.eq" class="h-4 w-4" />
            <IconNeq
                v-if="slotProps.option === Condition.neq"
                class="h-4 w-4"
            />
            <IconGe v-if="slotProps.option === Condition.ge" class="h-4 w-4" />
            <IconGt v-if="slotProps.option === Condition.gt" class="h-4 w-4" />
            <IconBetween
                v-if="slotProps.option === Condition.between"
                class="h-4 w-4"
            />
            <IconNull
                v-if="slotProps.option === Condition.isNull"
                class="h-4 w-4"
            />
            <IconNotNull
                v-if="slotProps.option === Condition.isNotNull"
                class="h-4 w-4"
            />
        </template>
    </SelectButton>
    <SelectButton
        v-else
        :modelValue="selectedCondition"
        :options="[
            Condition.lt,
            Condition.le,
            Condition.eq,
            Condition.neq,
            Condition.ge,
            Condition.gt,
            Condition.between,
            Condition.isNull,
            Condition.isNotNull,
        ]"
        @update:modelValue="updateCondition"
    >
        <template #option="slotProps">
            <IconLt v-if="slotProps.option === Condition.lt" class="h-4 w-4" />
            <IconLe v-if="slotProps.option === Condition.le" class="h-4 w-4" />
            <IconEq v-if="slotProps.option === Condition.eq" class="h-4 w-4" />
            <IconNeq
                v-if="slotProps.option === Condition.neq"
                class="h-4 w-4"
            />
            <IconGe v-if="slotProps.option === Condition.ge" class="h-4 w-4" />
            <IconGt v-if="slotProps.option === Condition.gt" class="h-4 w-4" />
            <IconBetween
                v-if="slotProps.option === Condition.between"
                class="h-4 w-4"
            />
            <IconNull
                v-if="slotProps.option === Condition.isNull"
                class="h-4 w-4"
            />
            <IconNotNull
                v-if="slotProps.option === Condition.isNotNull"
                class="h-4 w-4"
            />
        </template>
    </SelectButton>
</template>
